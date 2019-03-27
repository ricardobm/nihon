use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// An element in a diff between a source and an input, where the
/// source is canonical and the diff transforms the input to source.
///
/// The sequence of `Diff` entries give the edit operations that must
/// be applied to the `input` in order to produce `source`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Diff {
    /// Sequence of text is the same in source and input.
    ///
    /// Operation: just advance both `input` and `source` by the text
    /// length.
    Same(String),

    /// Sequence of text in the input that has no correspondence in
    /// the source.
    ///
    /// Operation: delete the given text from the current position in
    /// `input`, maintain the position.
    Delete(String),

    /// Sequence of text in the source that is missing in the input.
    ///
    /// Operation: insert the given text in `input` at the current
    /// position and advance the position to the end of the inserted
    /// text.
    Insert(String),

    /// The first string is the text in the input that corresponds
    /// to the text in the source given by the second string.
    ///
    /// Note that since the diff is performed by syllables, the
    /// texts between the source and input may share a common prefix
    /// and suffix.
    ///
    /// Operation: a `Change(a, b)` is equivalent to a `Delete(a)`
    /// followed by an `Insert(b)`.
    Change(String, String),
}

/// Returns a diff between the syllables in `source` and the text in
/// `input`.
///
/// The result is a sequence of `Diff` entries, corresponding to each
/// of the syllables in `source`:
///
/// - `Diff::Same`, `Diff::Change` and `Diff::Insert` correspond
///   one to one to the syllables in the source.
/// - `Diff::Delete` is for extraneous text that should be removed
///   from the input and as such do not have correspondence in the
///   source.
pub fn diff<'a, L: IntoIterator<Item = S>, S: AsRef<str>>(source: L, input: &'a str) -> Vec<Diff> {
    /*
    
    Objective
    =========
    
    Compute the minimum diff to go from `input` to `source`.
    
    Operations are applied relative to each syllable in the source
    and the cost of the diff operations are:
    
    - Same(txt)        => 0
    - Delete(txt)      => 1
    - Insert(txt)      => 1
    - Change(dst, src) => 1
    
    The `Delete` is the only operation that is applied exclusively
    to the input and as such is not bound to a syllable size.
    
    The fact that the `Insert` and `Change(_, src)` are bound to
    syllables prevents the algorithm to choosing trivial options
    (e.g. swapping whole words).
    
    Example:
    
        Source: 12345678
        Input:  1xx2yy467zz8
    
        Diff      Worst alternatives
        -------  ---------------------
        S(1)
        D(xx)    C(xx,2)+D(2) I(2)+D(xx2...)
        S(2)
        C(yy,3)  D(yy)+I(3)
        S(4)
        I(5)
        S(6)
        S(7)
        D(zz)    C(zz,8)+D(8) I(8)+D(zz8)
        S(8)
        -------
        Cost 4
    
    The algorithm
    =============
    
    Given `A = { a0, a1, a2 }` as the source syllables and `B` as
    the input string, we want to compute `op(a, b) -> (X, C)` where:
    
    - `a` is an index into `A`
    - `b` is an offset in `B`
    - `X` is the optimal operation in the diff, one of
      - `S(1)` -> Skip `A[a]` and `len(A[a])` in `B`
      - `D(n)` -> Delete `n` elements from `B`
      - `I(1)` -> Insert `A[a]` into `B` and skip it
      - `R(n)` -> Equivalent to `D(n)` followed by `I(1)`
    - `C` is the total cost of the diff for the current
    
    To implement `op` lets define a `cost` function that returns
    just `C`:
    
        cost(a, b)
            let A = A[a..], B = B[b..]
            if A is []
                1                          // delete B in a single op
            else if B is ""
                A.len                      // insert each in A
            else if B.has_prefix(A)
                cost(a + 1, b + A[0].len)  // skip prefix
            else min of
                ins = 1 + cost(a + 1, b)
                del = 1 + cost(a, b + k)      for k in 1 to B.len
                rep = 1 + cost(a + 1, b + k)  for k in 1 to B.len
    
    Then `op` can be defined as just returning the respective
    operation for the minimal cost.
    
    The actual cost is more complex than what is depicted above,
    see below for details.
    
    */

    #[derive(Copy, Clone, Debug)]
    enum D {
        End,
        Same,
        Insert,
        Delete(usize),
        Replace(usize),
    };

    struct Env<'a, S: AsRef<str>> {
        source: Vec<S>,
        input: &'a str,
        memo: HashMap<(usize, usize), (D, usize)>,
    };

    let mut env = Env {
        source: source.into_iter().collect(),
        input: input,
        memo: HashMap::new(),
    };

    fn op<'a, S: AsRef<str>>(env: &mut Env<'a, S>, a: usize, b: usize) -> (D, usize) {
        let key = (a, b);
        {
            if let Some(&res) = env.memo.get(&key) {
                return res;
            }
        }

        /*
        How costs are calculated:
        
        We calculate cost by the number of characters touched by the
        operations, so:
        
        - Insert the cost is the size of the inserted text
        - Delete the cost is the size of the deleted text
        - Change the cost is the size of the deleted + inserted
        
        Besides the cost above, we add +1 for each operation. This is
        so we prioritize the minimal number of operations.
        */

        let out = if a >= env.source.len() && b >= env.input.len() {
            // Empty case
            println!(">> @({}, {}) - END", a, b);
            (D::End, 0)
        } else if a >= env.source.len() {
            // We are at the end of source, so just delete the input
            // extra suffix in a single operation.
            let del_len = env.input.len() - b;
            println!(">> @({}, {}) - DEL", a, b);
            (D::Delete(del_len), del_len + 1)
        } else if b >= env.input.len() {
            // We are at the end of the input, so we append all
            // remaining syllables in the source, one at a time.
            let remaining = env.source.len() - a;
            let mut len = 0;
            for it in &env.source[a..] {
                len += it.as_ref().len();
            }
            println!(">> @({}, {}) - INS", a, b);
            (D::Insert, len + remaining)
        } else if env.input[b..].starts_with(env.source[a].as_ref()) {
            // Source and input match, skip the syllable and continue.
            let syllable_len = { env.source[a].as_ref().len() };
            let (_, cost) = op(env, a + 1, b + syllable_len);
            println!(">> @({}, {}) - SAME", a, b);
            (D::Same, cost)
        } else {
            let rem_input = { env.input.len() - b };
            let a_len = env.source[a].as_ref().len();

            // Cost of insertion.
            let ins = {
                let (_, cost) = op(env, a + 1, b);
                (D::Insert, cost + a_len + 1)
            };

            // Cost of deletion.
            let del = {
                let mut del = {
                    let (_, cost) = op(env, a, b + 1);
                    (1, cost + 1)
                };
                for k in 2..rem_input + 1 {
                    let (_, new_cost) = op(env, a, b + k);
                    let new_cost = new_cost + k;
                    if new_cost < del.1 {
                        del = (k, new_cost);
                    }
                }
                (D::Delete(del.0), del.1 + 1)
            };

            // Cost of replace.
            let rep = {
                let mut rep = {
                    let (_, cost) = op(env, a + 1, b + 1);
                    (1, cost + 1 + a_len)
                };
                for k in 2..rem_input + 1 {
                    let (_, new_cost) = op(env, a + 1, b + k);
                    let new_cost = new_cost + k + a_len;
                    if new_cost < rep.1 {
                        rep = (k, new_cost);
                    }
                }
                (D::Replace(rep.0), rep.1 + 1)
            };

            println!(
                ">> @({}, {}) - INS / DEL / REP: {:?} / {:?} / {:?}",
                a, b, ins, del, rep
            );

            // Minimize the cost. Precedence order is REP > DEL > INS
            if ins.1 < del.1 {
                if ins.1 < rep.1 {
                    ins
                } else {
                    rep
                }
            } else {
                if del.1 < rep.1 {
                    del
                } else {
                    rep
                }
            }
        };

        println!("== @({}, {}) = {:?}", a, b, out);

        env.memo.insert(key, out);
        out
    }

    let mut out = Vec::new();

    let mut a = 0;
    let mut b = 0;
    'main: loop {
        let (next, _) = { op(&mut env, a, b) };
        match next {
            D::End => {
                break 'main;
            }
            D::Same => {
                let s = env.source[a].as_ref();
                out.push(Diff::Same(s.to_string()));
                b += s.len();
                a += 1;
            }
            D::Insert => {
                let s = env.source[a].as_ref();
                out.push(Diff::Insert(s.to_string()));
                a += 1;
            }
            D::Delete(n) => {
                let s = &env.input[b..b + n];
                out.push(Diff::Delete(s.to_string()));
                b += s.len();
            }
            D::Replace(n) => {
                let s1 = &env.input[b..b + n];
                let s2 = env.source[a].as_ref();
                out.push(Diff::Change(s1.to_string(), s2.to_string()));
                a += 1;
                b += s1.len();
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_diff() {
        let src = vec!["A1", "B2", "C3", "D4", "E5", "F6", "G7", "H8"];
        assert_eq!(
            diff(&src, "A1xxB2yyD4F6G7zzH8"),
            vec![
                s("A1"),
                d("xx"),
                s("B2"),
                c("yy", "C3"),
                s("D4"),
                i("E5"),
                s("F6"),
                s("G7"),
                d("zz"),
                s("H8"),
            ]
        );
    }

    #[test]
    fn test_diff_empty() {
        // Empty source and input
        let src = Vec::<String>::new();
        assert_eq!(diff(&src, ""), Vec::new());

        // Empty source and non-empty input
        assert_eq!(diff(&src, "abc"), vec![d("abc")]);

        // Empty input
        let src = vec!["abc"];
        assert_eq!(diff(&src, ""), vec![i("abc")]);

        // Empty input, multiple syllables
        let src = vec!["a", "b", "c"];
        assert_eq!(diff(&src, ""), vec![i("a"), i("b"), i("c")]);
    }

    #[test]
    fn test_diff_insert() {
        let src = vec!["a", "b", "c"];
        assert_eq!(diff(&src, "bc"), vec![i("a"), s("b"), s("c")]);
        assert_eq!(diff(&src, "ab"), vec![s("a"), s("b"), i("c")]);
        assert_eq!(diff(&src, "ac"), vec![s("a"), i("b"), s("c")]);

        let src = vec!["a", "b", "c", "d", "e"];
        assert_eq!(
            diff(&src, "bd"),
            vec![i("a"), s("b"), i("c"), s("d"), i("e")]
        );
    }

    #[test]
    fn test_diff_delete() {
        let src = vec!["b", "c"];
        assert_eq!(diff(&src, "abc"), vec![d("a"), s("b"), s("c")]);
        assert_eq!(diff(&src, "[a]bc"), vec![d("[a]"), s("b"), s("c")]);
        assert_eq!(diff(&src, "bcd"), vec![s("b"), s("c"), d("d")]);
        assert_eq!(diff(&src, "bc[d]"), vec![s("b"), s("c"), d("[d]")]);
        assert_eq!(diff(&src, "bxc"), vec![s("b"), d("x"), s("c")]);
        assert_eq!(diff(&src, "b[x]c"), vec![s("b"), d("[x]"), s("c")]);

        let src = vec!["a", "b", "c", "d", "e", "f"];
        assert_eq!(
            diff(&src, "x[ab2cd3ef]x"),
            vec![
                d("x["),
                s("a"),
                s("b"),
                d("2"),
                s("c"),
                s("d"),
                d("3"),
                s("e"),
                s("f"),
                d("]x")
            ]
        );
    }

    #[test]
    fn test_diff_replace() {
        let src = vec!["a", "b", "c"];
        assert_eq!(diff(&src, "Abc"), vec![c("A", "a"), s("b"), s("c")]);
        assert_eq!(diff(&src, "aBc"), vec![s("a"), c("B", "b"), s("c")]);
        assert_eq!(diff(&src, "abC"), vec![s("a"), s("b"), c("C", "c")]);
        assert_eq!(diff(&src, "[A]bc"), vec![c("[A]", "a"), s("b"), s("c")]);
        assert_eq!(diff(&src, "a[B]c"), vec![s("a"), c("[B]", "b"), s("c")]);
        assert_eq!(diff(&src, "ab[C]"), vec![s("a"), s("b"), c("[C]", "c")]);

        let src = vec!["a", "b", "c", "d", "e", "f", "g"];
        assert_eq!(
            diff(&src, "[A]b[C]d[E]f[G]"),
            vec![
                c("[A]", "a"),
                s("b"),
                c("[C]", "c"),
                s("d"),
                c("[E]", "e"),
                s("f"),
                c("[G]", "g")
            ]
        );
    }

    fn s(txt: &str) -> Diff {
        Diff::Same(String::from(txt))
    }

    fn d(txt: &str) -> Diff {
        Diff::Delete(String::from(txt))
    }

    fn i(txt: &str) -> Diff {
        Diff::Insert(String::from(txt))
    }

    fn c(a: &str, b: &str) -> Diff {
        Diff::Change(String::from(a), String::from(b))
    }
}
