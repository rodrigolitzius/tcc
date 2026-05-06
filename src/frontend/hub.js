document.addEventListener('DOMContentLoaded', function() {
    const token = localStorage.getItem('Token');
    const server_url = localStorage.getItem('server_url');

    if (!token || !server_url) {
        window.location.replace('index.html');
        return;
    }

    document.getElementById('backend-url-display').textContent = server_url;

    document.getElementById('logout-btn').addEventListener('click', function() {
        localStorage.removeItem('Token');
        localStorage.removeItem('server_url');
        window.location.replace('index.html');
    });
});
