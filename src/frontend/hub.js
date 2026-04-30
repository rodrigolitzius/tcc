document.addEventListener('DOMContentLoaded', () => {
    const token = localStorage.getItem('token');
    const server_url = localStorage.getItem('server_url');

    if (!token || !server_url) {
        window.location.href = 'index.html';
        return;
    }

    document.getElementById('backend-url-display').textContent = server_url;

    document.getElementById('logout-btn').addEventListener('click', () => {
        localStorage.removeItem('token');
        localStorage.removeItem('server_url');
        window.location.href = 'index.html';
    });
});