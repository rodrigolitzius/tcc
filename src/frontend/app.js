const form = document.getElementById('loginform');

form.addEventListener('submit', function(event) {
    event.preventDefault();

    const nd_url = document.getElementById('navidrome-url').value;
    const server_url = document.getElementById('server-url').value;
    const nome = document.getElementById('nome').value;
    const senha = document.getElementById('senha').value;

    if (!nd_url || !server_url || !nome || !senha) {
        console.error('Preencha todos os campos');
        return;
    }

    fetch(server_url + '/login', {
        method: 'POST',
        headers: {
            'content-type': 'application/json'
        },
        body: JSON.stringify({
            username: nome,
            password: senha,
            url: nd_url
        })
    })
        .then(response => {
            if (!response.ok) throw new Error(`HTTP ${response.status}`);
            return response.json();
        })
        .then(data => {
            console.log('resposta do backend:', data);

            if (data.token) {
                localStorage.setItem('Token', data.token);
                console.log('Token salvo com sucesso');

                fetch(server_url + '/rota-protegida', {
                    method: 'GET',
                    headers: {
                        'Authorization': data.token
                    }
                })
                    .then(response => {
                        if (!response.ok) throw new Error(`HTTP ${response.status}`);
                        return response.json();
                    })
                    .then(data => console.log('rota protegida:', data))
                    .catch(error => console.error('erro na rota protegida:', error));
            } else {
                console.warn('Login falhou: token não recebido');
                alert('Login falhou. Verifique suas credenciais.');
            }
        })
        .catch(error => {
            console.error('erro:', error);
            alert('Erro ao conectar com o servidor.');
        });
});