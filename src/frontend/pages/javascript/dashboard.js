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

    fetch(server_url + '/recent', {
        headers: { 'Authorization': token }
    })
        .then(function (r) { return r.json(); })   // recebe json agora rodri
        .then(function (dados) {
            const itens = dados.slice(0, 6);

            if (itens.length === 0) {
                lista.innerHTML = '<li style="color:#94a3b8;">Nenhuma música encontrada.</li>';
                return;
            }

            lista.innerHTML = '';
            itens.forEach(function (musica) {
                const titulo = musica.title || 'sem título';
                const artista = musica.artist || '';
                const album = musica.album || '';
                const li = document.createElement('li');
                li.className = 'music-item';
                li.innerHTML =
                    // TODO: trocar o emoji por <img src="URL_DA_CAPA"> quando tiver o endpoint
                    '<div class="album-cover">🎵</div>' +
                    '<div class="music-info">' +
                        '<div class="music-title">' + titulo + '</div>' +
                        '<div class="music-artist">' + artista + '</div>' +
                        '<div class="music-album">' + album + '</div>' +
                    '</div>';
                lista.appendChild(li);
            });
        })
        .catch(function (err) {
            lista.innerHTML = '<li style="color:#f87171;">Erro: ' + err.message + '</li>';
        });
});
