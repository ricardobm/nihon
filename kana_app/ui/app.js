(function() {

    initComponents();
    handleRefresh();

    let app = new Vue({
        el: '#app',

        data: {
            model: {
                set: '',
            },
        },

        template: [
            '<div v-if="model.at_start">',
            '    <start-menu v-model="model.set" @selected="start"></start-menu>',
            '</div>',
        ].join('\n'),

        methods: {
            start: function(size) {
                main.send({ Start: { set: this.model.set, size: size }});
            }
        }
    });


    main.onMessage(function(msg) {
        if (msg.Refresh) {
            reload();
        } else if (msg.Update) {
            console.log('UPDATE', msg.Update);
            Vue.set(app, 'model', msg.Update);
        } else {
            console.log(msg);
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
                '        <a href="#" class="button" @click="$emit(\'selected\', 100)">100</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 200)">200</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 300)">300</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 400)">400</a>',
                '        <a href="#" class="button" @click="$emit(\'selected\', 500)">500</a>',
                '    </div>',
                '</div>'
            ].join('\n'),
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
