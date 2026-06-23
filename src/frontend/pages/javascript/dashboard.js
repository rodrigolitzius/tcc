document.addEventListener('DOMContentLoaded', function () {
    const token = localStorage.getItem('Token');
    const server_url = localStorage.getItem('server_url');

    if (!token || !server_url) {
        window.location.replace('../index.html');
        return;
    }

    document.getElementById('back-btn').addEventListener('click', function () {
        window.location.replace('../pages/hub.html');
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
        .then(function (r) { return r.json(); })
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

                const coverDiv = document.createElement('div');
                coverDiv.className = 'album-cover';
                coverDiv.innerHTML = '🎵';

                fetch(server_url + '/relay/getCoverArt?id=' + musica.id + '&size=400', {
                    headers: { 'Authorization': token }
                })
                    .then(function (r) {
                        if (!r.ok) throw new Error('erro');
                        return r.blob();
                    })
                    .then(function (blob) {
                        const url = URL.createObjectURL(blob);
                        const img = document.createElement('img');
                        img.src = url;
                        img.style = 'width:110px;height:100px;object-fit:cover;border-radius:10px;';
                        coverDiv.innerHTML = '';
                        coverDiv.appendChild(img);
                    })
                    .catch(function () {
                        coverDiv.innerHTML = '🎵';
                    });

                const infoDiv = document.createElement('div');
                infoDiv.className = 'music-info';
                infoDiv.innerHTML =
                    '<div class="music-title">' + titulo + '</div>' +
                    '<div class="music-artist">' + artista + '</div>' +
                    '<div class="music-album">' + album + '</div>';

                li.appendChild(coverDiv);
                li.appendChild(infoDiv);
                lista.appendChild(li);
            });
        })
        .catch(function (err) {
            lista.innerHTML = '<li style="color:#f87171;">Erro: ' + err.message + '</li>';
        });
});
