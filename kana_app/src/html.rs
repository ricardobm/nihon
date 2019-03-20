use regex::Regex;

// Matches a source file name and return its contents from the `ui`
// directory.
//
// The source file will be inlined, but on debug builds this will
// first attempt to load the file directly.
macro_rules! files {
    ( $src:ident $( , $x:expr )* ) => {
        match $src {
            $( $x => get_source($x, include_str!(concat!("../ui/", $x))), )*
            _ => String::new(),
        }
    };
}

macro_rules! html {
    ( $index:expr $( , $src:expr )* ) => {
        {
            use crate::html::*;

            fn content(src: &str) -> String {
                files!(src, $index $( , $src )* )
            }

            get_html(&content($index), content)
        }
    };
}

#[cfg(debug_assertions)]
pub fn get_source(file: &str, inline: &str) -> String {
    use std::fs::File;
    use std::io::Read;

    // On debug builds we attempt first to load the file from the
    // `kana_app/ui` directory.
    if let Ok(mut file) = File::open(format!("kana_app/ui/{}", file)) {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");
        return contents;
    }
    inline.to_string()
}

#[cfg(not(debug_assertions))]
pub fn get_source(_file: &str, inline: &str) -> String {
    // Release builds just return the inlined source.
    inline.to_string()
}

/// Replace `<link>` and `<script>` tags on the HTML text by their
/// source file contents by calling `include` to retrieve them.
pub fn get_html(html: &str, include: impl Fn(&str) -> String) -> String {
    lazy_static! {
        static ref RE_LINK: Regex = Regex::new(
            r#"(?xi)

            # Tag opening
            <link \s+

            # Attributes
            (?:
                \s*
                (?:
                    href="(?P<src> [^"]+)" # we just want this
                    | \w+(="[^"]*")?
                )
            )+

            # Tag closing
            \s* (?: >\s*</link> | /> )
            "#
        )
        .unwrap();
        static ref RE_SCRIPT: Regex = Regex::new(
            r#"(?xi)

            # Tag opening
            <script \s+

            # Attributes
            (?:
                \s*
                (?:
                    src="(?P<src> [^"]+)" # we just want this
                    | \w+(="[^"]*")?
                )
            )+

            # Tag closing
            \s* (?: >\s*</script> | /> )
            "#
        )
        .unwrap();
    }

    let html = RE_LINK.replace_all(html, |captures: &regex::Captures| -> String {
        let src = captures.name("src").map_or("", |m| m.as_str());
        format!(r#"<style type="text/css">{}</style>"#, include(src))
    });

    let html = RE_SCRIPT.replace_all(&html, |captures: &regex::Captures| -> String {
        let src = captures.name("src").map_or("", |m| m.as_str());
        format!(
            r#"<script type="text/javascript">{}</script>"#,
            include(src)
        )
    });

    html.to_string()
}
