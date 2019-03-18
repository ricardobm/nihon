setTimeout(function() {
    window.external.invoke('From JavaScript!');
}, 1000);

function showAlert(msg) {
    alert(msg);
}
