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
        localStorage.removeItem('Token');
        localStorage.removeItem('server_url');
        window.location.replace('../index.html');
    });

    // mapa nome-do-artista (normalizado) -> id, usado pra descobrir se o
    // subtitulo de artista nos cards de album/faixa pode virar link.
    // Só conseguimos resolver nomes que aparecem em /most-played/artists;
    // nomes fora dessa lista (feat., grafia diferente, pouco tocados) ficam
    // como texto normal, sem virar link.
    let mapaArtistas = {};

    function normaliza(nome) {
        return (nome || '').trim().toLowerCase();
    }

    function urlArtista(id, nome) {
        return 'artist.html?id=' + encodeURIComponent(id) + '&nome=' + encodeURIComponent(nome || '');
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

    // cards de ARTISTA usam <img> com object-fit:cover — resolve enquadramento pra qualquer proporcao
    function montaCardArtista(nome, plays) {
        const card = document.createElement('div');
        card.className = 'top-card top-card-artist';
        card.style.background = gradientePorNome(nome);

        // imagem — começa oculta, aparece quando carrega
        const img = document.createElement('img');
        img.className = 'top-card-img';
        img.alt = nome;
        card.appendChild(img);

        // inicial de fallback — some quando a imagem carrega
        const inicial = document.createElement('div');
        inicial.className = 'top-card-initial';
        inicial.textContent = inicialDoNome(nome);
        card.appendChild(inicial);

        const overlay = document.createElement('div');
        overlay.className = 'top-card-overlay';
        overlay.innerHTML =
            '<div class="top-card-name">' + nome + '</div>' +
            '<div class="top-card-plays">' + plays + (plays === 1 ? ' play' : ' plays') + '</div>';
        card.appendChild(overlay);

        return card;
    }

    function aplicaFotoArtista(card, artistId) {
        if (!artistId) return;

        fetch(server_url + '/relay/getCoverArt?id=' + artistId + '&size=400', {
            headers: { 'Authorization': token }
        })
            .then(function (r) {
                if (!r.ok) throw new Error('sem foto');
                return r.blob();
            })
            .then(function (blob) {
                const url = URL.createObjectURL(blob);
                const img = card.querySelector('.top-card-img');
                if (!img) return;
                img.onload = function () {
                    img.style.opacity = '1';
                    const inicial = card.querySelector('.top-card-initial');
                    if (inicial) inicial.remove();
                };
                img.src = url;
            })
            .catch(function () {});
    }

    // cards de ALBUM/FAIXA continuam usando background-image (capas sao sempre quadradas)
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

    function montaCard(nome, subtitulo, plays) {
        const card = document.createElement('div');
        card.className = 'top-card';

        const overlay = document.createElement('div');
        overlay.className = 'top-card-overlay';

        const nomeDiv = document.createElement('div');
        nomeDiv.className = 'top-card-name';
        nomeDiv.textContent = nome;
        overlay.appendChild(nomeDiv);

        if (subtitulo) {
            const subDiv = document.createElement('div');
            subDiv.className = 'top-card-sub';
            subDiv.textContent = subtitulo;

            // se o nome do artista bate com algum id conhecido, vira link
            const artistId = mapaArtistas[normaliza(subtitulo)];
            if (artistId) {
                subDiv.classList.add('artist-link');
                subDiv.addEventListener('click', function (ev) {
                    ev.stopPropagation();
                    window.location.href = urlArtista(artistId, subtitulo);
                });
            }

            overlay.appendChild(subDiv);
        }

        const playsDiv = document.createElement('div');
        playsDiv.className = 'top-card-plays';
        playsDiv.textContent = plays + (plays === 1 ? ' play' : ' plays');
        overlay.appendChild(playsDiv);

        card.appendChild(overlay);

        return card;
    }

    function carregaComCapaPropria(endpoint, elementId, montaSubtitulo) {
        const grid = document.getElementById(elementId);

        fetch(server_url + endpoint + '?limit=10', {
            headers: { 'Authorization': token }
        })
            .then(function (r) {
                if (!r.ok) throw new Error('erro ' + r.status);
                return r.json();
            })
            .then(function (dados) {
                if (!dados || dados.length === 0) {
                    grid.innerHTML = '<p style="color:#94a3b8;">Nenhum dado encontrado.</p>';
                    return;
                }

                grid.innerHTML = '';

                dados.forEach(function (item) {
                    const subtitulo = montaSubtitulo ? montaSubtitulo(item) : '';
                    const card = montaCard(item.name, subtitulo, item.plays);
                    grid.appendChild(card);
                    aplicaCapa(card, item.id);
                });
            })
            .catch(function (err) {
                grid.innerHTML = '<p style="color:#f87171;">Erro: ' + err.message + '</p>';
            });
    }

    function renderizaArtistas(artistas) {
        const grid = document.getElementById('grid-artistas');

        if (!artistas || artistas.length === 0) {
            grid.innerHTML = '<p style="color:#94a3b8;">Nenhum dado encontrado.</p>';
            return;
        }

        grid.innerHTML = '';

        artistas.slice(0, 9).forEach(function (artista, index) {
            const card = montaCardArtista(artista.name, artista.plays);
            if (index === 0) {
                card.classList.add('destaque');
            }
            card.addEventListener('click', function () {
                window.location.href = urlArtista(artista.id, artista.name);
            });
            grid.appendChild(card);
            aplicaFotoArtista(card, artista.id);
        });
    }

    // busca TODOS os artistas (sem limit) de uma vez: usa esse resultado pra
    // montar o grid de Top Artistas E pra preencher o mapaArtistas usado nos
    // subtitulos de album/faixa. So depois disso carrega albuns e faixas.
    fetch(server_url + '/most-played/artists', {
        headers: { 'Authorization': token }
    })
        .then(function (r) {
            if (!r.ok) throw new Error('erro ' + r.status);
            return r.json();
        })
        .then(function (artistas) {
            artistas = artistas || [];

            artistas.forEach(function (artista) {
                mapaArtistas[normaliza(artista.name)] = artista.id;
            });

            renderizaArtistas(artistas);
            carregaComCapaPropria('/most-played/albums', 'grid-albuns', function (item) { return item.artist; });
            carregaComCapaPropria('/most-played/tracks', 'grid-faixas', function (item) { return item.artist; });
        })
        .catch(function (err) {
            document.getElementById('grid-artistas').innerHTML = '<p style="color:#f87171;">Erro: ' + err.message + '</p>';
            // ainda assim tenta carregar albuns/faixas, so sem os links de artista
            carregaComCapaPropria('/most-played/albums', 'grid-albuns', function (item) { return item.artist; });
            carregaComCapaPropria('/most-played/tracks', 'grid-faixas', function (item) { return item.artist; });
        });
});