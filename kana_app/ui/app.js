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

                // Failures:
                fail: false,
                fail_word: '',
                wrong_answer: '',
                correct_answer: '',

            },

            paused: false,
            t0: 0,
        },

        template: [
            '<div ref="main" ',
            '    @keyup.pause.capture="toggle_pause" tabindex="0" ',
            '>',
            '    <start-menu ',
            '        v-show="model.page == \'Start\'" ',
            '        @selected="start" ',
            '        v-model="model.set" ',
            '    />',
            '    <wrong-answer ',
            '        v-show="model.fail && !paused" ',
            '        :word="model.fail_word" ',
            '        :wrong="model.wrong_answer" ',
            '        :correct="model.correct_answer" ',
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
            '        <p>Missed {{model.misses}} in {{model.hits}}</p>',
            '        <a href="#" v-on:click.stop.prevent="restart">[Back]</a>',
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
                '    <div v-for="it in options">',
                '        <input type="radio" :key="it.set" :id="it.set" :value="it.set" :checked="value == it.set" />',
                '        <label :for="it.set">{{it.text}}</label>',
                '    </div>',
                '    <div class="button-row">',
                '        <a href="#" class="button" @click="$emit(\'selected\', 10)">10</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 100)">100</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 200)">200</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 300)">300</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 400)">400</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 500)">500</a>',
                '    </div>',
                '</div>'
            ].join('\n'),
        });

        Vue.component('wrong-answer', {
            props: [
                'word',
                'correct',
                'wrong',
            ],
            template: [
                '<div class="wrong-answer">',
                '    <p><b>Word:</b> {{word}}</p>',
                '    <p><b>Expected:</b> {{correct}}</p>',
                '    <p><b>Was:</b> {{wrong}}</p>',
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
                '    <p class="japanese">{{word}}</p>',
                '    <input ref="input" type="text" v-model="text" v-on:keyup.enter="submit"/>',
                '    <a href="#" v-on:click.stop.prevent="restart">[Restart]</a>',
                '    <p>{{status}}</p>',
                '    <p>{{chars}}/{{total_chars}}</p>',
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
