(function() {

    let app = new Vue({
        el: '#app',
        data: {
            message: 'Hello Vue!',
        },
    });

    handleRefresh();

    main.onMessage(function(msg) {
        if (msg.Refresh) {
            reload();
        }
    })

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
