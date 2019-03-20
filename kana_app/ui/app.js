setTimeout(function() {
    window.external.invoke('From JavaScript!');
}, 1000);

function showAlert(msg) {
    document.body.innerHTML += '<div>' + msg + '</div>';
}

function reload(content) {
    let el = document.documentElement;
    el.innerHTML = content;

    let ls = el.getElementsByTagName('script');
    for (let i = 0; i < ls.length; i++) {
        eval(ls[i].text);
    }
}

document.onkeydown = function (e) {
    if (e.key === 'F5') {
        e.preventDefault();
        e.stopPropagation();
        window.external.invoke('refresh');
    }
}
