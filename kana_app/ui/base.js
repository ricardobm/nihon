
// Main interface to interact with the Rust app:
let main = {

    handlers: [],

    // Sends a generic message to the main Rust program.
    send: function(msg) {
        window.external.invoke(JSON.stringify(msg));
    },

    // This is invoked by the main Rust program to send a message
    // to the UI.
    exec: function(msg) {
        this.handlers.forEach(function(it) {
            setTimeout(function() {
                it(msg);
            });
        });
    },

    // Registers a handler called on any message from the Rust program.
    onMessage: function(fn) {
        this.handlers.push(fn);
    },
};

window.main = main;

(function() {

    // Redirect the JavaScript console to the Rust program output:

    window.console = {
        log: getLog(false),
        error: getLog(true),
    };

    function getLog(isError) {
        return function() {
            let str = Array.from(arguments).map(stringify).join(' ');
            if (isError) {
                main.send({ Error: str });
            } else {
                main.send({ Console: str });
            }
        }
    }

    function stringify(value) {
        if (typeof value === 'string' || value instanceof String) {
            return value;
        }
        if (typeof value === 'number') {
            return value.toString();
        }
        return JSON.stringify(value);
    }
}())


