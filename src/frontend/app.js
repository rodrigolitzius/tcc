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
    }).then(response => {
        if (!response.ok) {
            console.warn("Throwing error")
            const error = new Error(`HTTP ${response.status}`);
            error.code = response.status;
            error.message = response.json();

            throw error;
        }
    }).then(data => {
        console.log('resposta do backend:', data);

        if (data.id) {
            localStorage.setItem('Token', data.id);
            console.log('Token salvo com sucesso');

        } else {
            console.warn('Login falhou: token não recebido');
            alert('Login falhou: token não recebido');
        }
    }).catch(error => {
        console.error('erro:', error);

        if (error.code == 401) {
            alert('Acesso negado. Verifique suas credencias');
        } else if (error.code == 404) {
            alert('Servidor Navidrome não encontrado');
        } else {
            alert('Servidor retornou um erro');
        }
    });
});
