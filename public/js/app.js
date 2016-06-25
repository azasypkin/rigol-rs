'use strict';
(function() {
    document.getElementById('run-btn').addEventListener('click', function() {
        fetch('/run').then(function() {
            alert('Rigol is run!');
        }, function() {
            alert('Rigol is NOT run!');
        })
    });
})(this);