document.addEventListener('DOMContentLoaded', function () {
    const token = localStorage.getItem('Token');
    const server_url = localStorage.getItem('server_url');

    if (!token || !server_url) {
        window.location.replace('../index.html');
        return;
    }

    const params = new URLSearchParams(window.location.search);
    const artistId = params.get('id');
    const artistNomeFallback = params.get('nome') || '';

    document.getElementById('back-btn').addEventListener('click', function () {
        window.location.replace('top.html');
    });

    document.getElementById('logout-btn').addEventListener('click', function () {
        localStorage.removeItem('Token');
        localStorage.removeItem('server_url');
        window.location.replace('../index.html');
    });

    if (!artistId) {
        document.getElementById('artist-name').textContent = 'Artista não encontrado';
        document.getElementById('grid-albuns').innerHTML = '<p style="color:#f87171;">Nenhum ID de artista foi informado.</p>';
        return;
    }

    function gradientePorNome(nome) {
        let hash = 0;
        for (let i = 0; i < nome.length; i++) {
            hash = nome.charCodeAt(i) + ((hash << 5) - hash);
        }
        const hue1 = Math.abs(hash) % 360;
        const hue2 = (hue1 + 50) % 360;
        return 'linear-gradient(135deg, hsl(' + hue1 + ', 60%, 28%) 0%, hsl(' + hue2 + ', 55%, 16%) 100%)';
    }

    function inicialDoNome(nome) {
        return nome.trim().charAt(0).toUpperCase();
    }

    // traduz os valores crus do MusicBrainz (tipo/genero) pra exibicao em pt-BR
    function traduzTipo(tipo) {
        const mapa = {
            'Person': 'Artista solo',
            'Group': 'Banda',
            'Orchestra': 'Orquestra',
            'Choir': 'Coral',
            'Character': 'Personagem',
            'Other': 'Outro'
        };
        return mapa[tipo] || tipo;
    }

    function traduzGenero(genero) {
        const mapa = {
            'Male': 'Masculino',
            'Female': 'Feminino',
            'Other': 'Outro'
        };
        return mapa[genero] || genero;
    }

    function aplicaFotoArtista(nome) {
        const img = document.getElementById('artist-photo-img');
        const inicial = document.getElementById('artist-photo-initial');
        const banner = document.getElementById('artist-banner');

        banner.style.setProperty('--artist-gradient', gradientePorNome(nome));
        inicial.textContent = inicialDoNome(nome);

        fetch(server_url + '/relay/getCoverArt?id=' + artistId + '&size=600', {
            headers: { 'Authorization': token }
        })
            .then(function (r) {
                if (!r.ok) throw new Error('sem foto');
                return r.blob();
            })
            .then(function (blob) {
                const url = URL.createObjectURL(blob);
                img.onload = function () {
                    img.style.opacity = '1';
                    inicial.style.display = 'none';
                };
                img.src = url;
            })
            .catch(function () {});
    }

    function montaMeta(dados) {
        const meta = document.getElementById('artist-meta');
        const partes = [];

        if (dados.artist_type) {
            partes.push('<span class="artist-meta-tag">' + traduzTipo(dados.artist_type) + '</span>');
        }
        if (dados.gender) {
            partes.push('<span class="artist-meta-tag">' + traduzGenero(dados.gender) + '</span>');
        }
        partes.push('<span class="artist-meta-tag">' + dados.album_count + (dados.album_count === 1 ? ' álbum' : ' álbuns') + '</span>');

        meta.innerHTML = partes.join('');
    }

    function aplicaCapa(card, coverId) {
        if (!coverId) return;

        fetch(server_url + '/relay/getCoverArt?id=' + coverId + '&size=300', {
            headers: { 'Authorization': token }
        })
            .then(function (r) {
                if (!r.ok) throw new Error('sem capa');
                return r.blob();
            })
            .then(function (blob) {
                const url = URL.createObjectURL(blob);
                card.style.backgroundImage = 'url(' + url + ')';
            })
            .catch(function () {});
    }

    function montaCardAlbum(album) {
        const card = document.createElement('div');
        card.className = 'top-card';

        const overlay = document.createElement('div');
        overlay.className = 'top-card-overlay';
        overlay.innerHTML =
            '<div class="top-card-name">' + album.name + '</div>' +
            '<div class="top-card-sub">' + album.year + '</div>' +
            '<div class="top-card-plays">' + album.plays + (album.plays === 1 ? ' play' : ' plays') + '</div>';
        card.appendChild(overlay);

        return card;
    }

    function carregaArtista() {
        document.getElementById('artist-name').textContent = artistNomeFallback || 'Carregando...';

        fetch(server_url + '/artist/' + artistId, {
            headers: { 'Authorization': token }
        })
            .then(function (r) {
                if (!r.ok) throw new Error('erro ' + r.status);
                return r.json();
            })
            .then(function (dados) {
                document.getElementById('page-title').textContent = dados.name;
                document.getElementById('artist-name').textContent = dados.name;
                document.title = dados.name + ' — Navalyze';

                montaMeta(dados);
                aplicaFotoArtista(dados.name);

                const grid = document.getElementById('grid-albuns');

                if (!dados.albums || dados.albums.length === 0) {
                    grid.innerHTML = '<p style="color:#94a3b8;">Nenhum álbum encontrado.</p>';
                    return;
                }

                grid.innerHTML = '';
                dados.albums.forEach(function (album) {
                    const card = montaCardAlbum(album);
                    grid.appendChild(card);
                    aplicaCapa(card, album.id);
                });
            })
            .catch(function (err) {
                document.getElementById('artist-name').textContent = artistNomeFallback || 'Erro ao carregar artista';
                document.getElementById('grid-albuns').innerHTML = '<p style="color:#f87171;">Erro: ' + err.message + '</p>';
            });
    }

    carregaArtista();
});