document.addEventListener('DOMContentLoaded', function () {
    const token = localStorage.getItem('Token');
    const server_url = localStorage.getItem('server_url');

    if (!token || !server_url) {
        window.location.replace('../index.html');
        return;
    }

    document.getElementById('back-btn').addEventListener('click', function () {
        window.location.replace('../hub.html');
    });

    document.getElementById('logout-btn').addEventListener('click', function () {
        localStorage.removeItem('token');
        localStorage.removeItem('server_url');
        window.location.replace('../index.html');
    });

    const lista = document.getElementById('lista-musicas');

    fetch(server_url + '/dev/recent', {
        headers: { 'Authorization': token }
    })
        .then(function (r) { return r.text(); })   // texto puro, não json
        .then(function (texto) {
            const linhas = texto.trim().split('\n').filter(Boolean).slice(0, 6);

            if (linhas.length === 0) {
                lista.innerHTML = '<li style="color:#94a3b8;">Nenhuma música encontrada.</li>';
                return;
            }

            lista.innerHTML = '';
            linhas.forEach(function (linha) {
                const match = linha.match(/"(.+?)"\s*-\s*"(.+?)"/);
                const titulo = match ? match[1] : linha;
                const artista = match ? match[2].split('•')[0].trim() : '';

                const li = document.createElement('li');
                li.className = 'music-item';
                li.innerHTML =
                    // TODO: trocar o emoji por <img src="URL_DA_CAPA"> quando tiver o endpoint
                    '<div class="album-cover">🎵</div>' +
                    '<div class="music-info">' +
                        '<span class="music-title">' + titulo + '</span>' +
                        '<span class="music-artist">' + artista + '</span>' +
                    '</div>';
                lista.appendChild(li);
            });
        })
        .catch(function (err) {
            lista.innerHTML = '<li style="color:#f87171;">Erro: ' + err.message + '</li>';
        });
});