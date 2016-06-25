'use strict';
(function() {
    document.getElementById('run-btn').addEventListener('click', function() {
        fetch('/run').then(function(response) {
            return response.blob();
        }).then(function(blob) {
            alert('Rigol is run!');

            document.getElementById('screenshot-img').src = URL.createObjectURL(blob);
        }, function() {
            alert('Rigol is NOT run!');
        })
    });
})(this);