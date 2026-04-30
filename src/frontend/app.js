const form = document.getElementById('loginform');
const btn = document.getElementById('login-btn');

if (localStorage.getItem('Token')) {
    window.location.href = 'hub.html';
}

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

    const textooriginal = btn.textContent;
    btn.textContent = 'Entrando...';
    btn.disabled = true;

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
    }).then(function(response) {
        if (!response.ok) {
             return response.json().catch(() => ({})).then(function(data) {
            const error = new Error(`HTTP ${response.status}`);
            error.code = response.status;
            

            throw error;
        });
        }
        return response.json().then(function(json) {
            if (json.id) {
                localStorage.setItem('Token', json.id);
                console.log('Token salvo com sucesso');

            } else {
                console.warn('Login falhou: token não recebido');
                alert('Login falhou: token não recebido');
            }
        })

    }).catch(function(error) {
        if (error.code == 401) {
            alert('Acesso negado. Verifique suas credencias');
        } else if (error.code == 404) {
            alert('Servidor Navidrome não encontrado');
        } else {
            alert('Servidor retornou um erro');
        }
    }).finally(function() {
        btn.textContent = textooriginal;
        btn.disabled = false;
    });
});