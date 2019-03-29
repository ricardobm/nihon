(function() {

    // Pause Break
    Vue.config.keyCodes.pause = 19;

    initComponents();
    handleRefresh();

    let app = new Vue({
        el: '#app',

        data: {
            model: {
                set: '',
                page: '',

                // Training:

                word: '',
                hits: 0,
                misses: 0,
                remaining: 0,
                chars_total: 0,
                chars_done: 0,
                answer_time: 0,

                // Failures:
                submitted: null,
            },

            paused: false,
            t0: 0,
        },

        computed: {

            fail: function() {
                return this.model.submitted && !this.model.submitted.is_match;
            },

            answer_time_text: function() {
                let dur = Math.round(this.model.answer_time / 1000);
                let min = Math.floor(dur / 60);
                let sec = dur - min * 60;
                let str = '';
                let sep = '';
                if (min > 0) {
                    str += '<b>' + min.toString() + '</b> minute' + (min != 1 ? 's' : '');
                    sep = ' and ';
                }
                if (str == '' || sec > 0) {
                    str += sep + '<b>' + sec.toString() + '</b> second' + (sec != 1 ? 's' : '');
                }
                return str;
            },

            answer_percent: function() {
                let pc = Math.round(100 * this.model.misses / this.model.hits);
                return pc;
            },

            answer_emoji: function() {
                let pc = this.answer_percent;
                let emoji = '';
                if (this.model.misses === 0) {
                    emoji = '🤗💯🎉';
                } else if (pc <= 10) {
                    emoji = '😃';
                } else if (pc <= 20) {
                    emoji = '🙂';
                } else if (pc <= 30) {
                    emoji = '🙃';
                } else if (pc <= 40) {
                    emoji = '🤨';
                } else if (pc <= 50) {
                    emoji = '😐';
                } else if (pc <= 60) {
                    emoji = '😑';
                } else if (pc <= 70) {
                    emoji = '😒';
                } else if (pc <= 80) {
                    emoji = '😟';
                } else if (pc <= 90) {
                    emoji = '😦';
                } else if (pc <= 100) {
                    emoji = '😰';
                } else {
                    emoji = '😭💩';
                }
                return emoji;
            },
        },

        template: [
            '<div ref="main" class="main"',
            '    @keyup.pause.capture="toggle_pause" tabindex="0" ',
            '>',
            '    <start-menu ',
            '        v-show="model.page == \'Start\'" ',
            '        @selected="start" ',
            '        v-model="model.set" ',
            '    />',
            '    <wrong-answer ',
            '        v-show="fail && !paused" ',
            '        :model="model.submitted" ',
            '    />',
            '    <training-card ref="training" ',
            '        v-show="model.page == \'Training\' && !paused" ',
            '        @submit="submit" ',
            '        @restart="restart" ',
            '        :word="model.word" ',
            '        :hits="model.hits" ',
            '        :misses="model.misses" ',
            '        :remaining="model.remaining" ',
            '        :chars="model.chars_done" ',
            '        :total_chars="model.chars_total" ',
            '    />',
            '    <div v-show="model.page == \'Training\' && paused">',
            '        <h1>Paused</h1>',
            '    </div>',
            '    <div class="summary" ',
            '        v-show="model.page == \'Summary\'" ',
            '    >',
            '        <div>',
            '            <h1>Complete <span class="emoji" style="font-size: 0.8em" v-html="answer_emoji"></span></h1>',
            '            <hr/>',
            '            <p>',
            '                Finished with…',
            '            </p>',
            '            <p>',
            '                <span class="tab" /><b class="num">{{model.hits}}</b> words',
            '            </p>',
            '            <p>',
            '                <span class="tab" /><b class="num">{{model.chars_total}}</b> characters',
            '            </p>',
            '            <p>',
            '                <span class="tab" />',
            '                <span v-if="model.misses != 1">',
            '                    <b class="num">{{model.misses}}</b> mistakes',
            '                </span>',
            '                <span v-if="model.misses == 1">',
            '                    <b class="num">{{model.misses}}</b> mistake',
            '                </span>',
            '                <span v-if="model.misses > 0">',
            '                    <em>({{answer_percent}}%)</em>',
            '                </span>',
            '            </p>',
            '            <hr/>',
            '            <p>',
            '                Completed in <span v-html="answer_time_text"></span>.',
            '            </p>',
            '            <hr/>',
            '            <p style="font-size: 0.6em">',
            '                Missing from set:',
            '            </p>',
            '            <p style="font-size: 0.6em; padding: 0 5vw 0 5vw; text-align: center">',
            '                <span class="japanese" v-for="it in model.missing">',
            '                    {{it}}',
            '                </span>',
            '            </p>',
            '        </div>',
            '        <a href="#" class="restart" v-on:click.stop.prevent="restart">[Back to Menu]</a>',
            '    </div>',
            '</div>',
        ].join('\n'),

        methods: {
            start: function(size) {
                this.reset_timer();
                main.send({ Start: { set: this.model.set, size: size }});
            },

            restart: function() {
                main.send({ Restart: null });
            },

            submit: function(text) {
                let delta = this.get_timer();
                this.reset_timer();
                main.send({ Submit: { text: text, elapsed_ms: delta } });
            },

            toggle_pause: function() {
                var me = this;
                this.reset_timer();
                if (this.model.page === 'Training') {
                    this.paused = !this.paused;
                    setTimeout(function() {
                        if (me.paused) {
                            me.$refs.main.focus();
                        } else {
                            let input = me.$refs.training.$refs.input;
                            input.value = '';
                            input.focus();
                        }
                    });

                } else {
                    this.paused = false;
                }
            },

            reset_timer: function() {
                this.t0 = Date.now();
            },

            get_timer: function() {
                return Math.round(Date.now() - this.t0);
            },
        }
    });


    main.onMessage(function(msg) {
        if (msg.Refresh) {
            reload();
        } else if (msg.Update) {
            console.log('UPDATE', msg.Update);
            Vue.set(app, 'model', msg.Update);
            app.reset_timer();
        } else {
            console.log('Invalid message:', msg);
        }
    });

    // Init the application
    main.send({ Init: null });

    function initComponents() {

        Vue.component('start-menu', {
            props:['value'],
            data: function() {
                return {
                    options: [
                        { set: 'Hiragana', text: 'Hiragana' },
                        { set: 'Katakana', text: 'Katakana' },
                        { set: 'All',      text: 'Hiragana + Katakana' },
                        { set: 'Rare',     text: 'Hiragana + Katakana + Rare' },
                    ],
                };
            },
            template: [
                '<div @change="$emit(\'input\', $event.target.value)" class="start-menu">',
                '    <h1>Choose your training</h1>',
                '    <div class="menu-row" v-for="it in options">',
                '        <input type="radio" :key="it.set" :id="it.set" :value="it.set" :checked="value == it.set" />',
                '        <label :for="it.set">{{it.text}}</label>',
                '    </div>',
                '    <div class="button-row">',
                '        <a href="#" class="button" @click="$emit(\'selected\',  10)">10</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\',  50)">50</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 100)">100</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 250)">250</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 500)">500</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\',   0)">All</a>',
                '    </div>',
                '</div>'
            ].join('\n'),
        });

        Vue.component('wrong-answer', {
            props: [
                'model',
            ],
            computed: {
                data: function() {
                    return this.model || {};
                },
                diff: function() {
                    let model = this.model;
                    if (!model || !model.diff) {
                        return {};
                    }

                    let kana = '';
                    let actual = '';
                    let answer = '';
                    let kana_index = 0;
                    for (let i = 0; i < model.diff.length; i++) {
                        let it = model.diff[i];
                        if (it.Same) {
                            kana   += model.split[kana_index];
                            actual += it.Same;
                            answer += it.Same;
                            kana_index++;
                        } else if (it.Delete) {
                            answer += del(it.Delete);
                        } else if (it.Insert) {
                            kana   += ins(model.split[kana_index]);
                            actual += ins(it.Insert);
                            kana_index++;
                        } else if (it.Change) {
                            let src = it.Change[0];
                            let dst = it.Change[1];
                            kana   += rep(model.split[kana_index]);
                            actual += rep(dst);
                            answer += rep(src);
                            kana_index++;
                        }
                    }

                    return {
                        kana:   kana,
                        actual: actual,
                        answer: answer,
                    }

                    function del(txt) {
                        return '<span class="diff-del">' + txt + '</span>'
                    }

                    function ins(txt) {
                        return '<span class="diff-ins">' + txt + '</span>'
                    }

                    function rep(txt) {
                        return '<span class="diff-rep">' + txt + '</span>'
                    }
                }
            },
            template: [
                '<div class="wrong-answer">',
                '    <p><b>word:</b> <span class="japanese" v-html="diff.kana"></span></p>',
                '    <p><b>expected:</b> <span class="mono" v-html="diff.actual"></span></p>',
                '    <p><b>was:</b> <span class="mono" v-html="diff.answer"></span></p>',
                '</div>',
            ].join('\n'),
        });

        Vue.component('training-card', {
            props: [
                'word',
                'hits',
                'misses',
                'remaining',
                'chars',
                'total_chars',
            ],

            data: function() {
                return { text: '' };
            },

            computed: {
                status: function() {
                    let total = this.hits + this.remaining;
                    let s = this.hits + '/' + total;
                    if (this.misses > 0) {
                        s += ' (' + this.misses;
                        if (this.misses === 1) {
                            s += ' miss';
                        } else {
                            s += ' misses';
                        }
                        s += ')';
                    }
                    return s;
                },

                percent_complete: function() {
                    let pc = (100 * this.chars / this.total_chars).toFixed(1);
                    return pc;
                },
            },

            methods: {
                submit: function() {
                    this.$emit('submit', this.text);
                    this.text = '';
                },

                restart: function() {
                    this.text = '';
                    this.$emit('restart');
                },
            },

            template: [
                '<div class="training-card">',
                '    <p class="word japanese">{{word}}</p>',
                '    <input ref="input" type="text" v-model="text" v-on:keyup.enter="submit"/>',
                '    <p class="status">{{status}}</p>',
                '    <div class="progress">',
                '        <div class="progress-text">{{percent_complete}}%</div>',
                '        <div class="progress-bar" ',
                '            :style="{ width: percent_complete + \'%\' }"',
                '        >',
                '            <div class="progress-text">{{percent_complete}}%</div>',
                '            <div class="bar">&nbsp;</div>',
                '        </div>',
                '    </div>',
                '    <a href="#" class="restart" v-on:click.stop.prevent="restart">[Back to Menu]</a>',
                '</div>',
            ].join('\n'),

            updated: function() {
                let me = this;
                setTimeout(function() {
                    me.$refs.input.focus();
                });
            }
        });
    }

    // We hijack the F5 to send a refresh message to the Rust app so
    // that it can reload the web resources on the server side.
    function handleRefresh() {
        document.onkeydown = function (e) {
            if (e.key === 'F5') {
                e.preventDefault();
                e.stopPropagation();
                main.send({ Refresh: null });
            }
        };
    }

    // Called to reload the whole document.
    function reload() {
        window.location.reload();
    }

}());
