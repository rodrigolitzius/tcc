const form =
document.getElementById('loginform')

form.addEventListener('submit', function
(event)
   {event.preventDefault();

const nd_url =
document.getElementById('navidrome-url').value;
const server_url =
document.getElementById('server-url').value;
const nome =
document.getElementById('nome').value;
const senha =
document.getElementById('senha').value;

//print = console.log
    console.log('NOME: ', nome);
    console.log('SENHA: ', senha);

// fetch(carteiro)

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
    }) //estudar .then
        .then(response => response.json()) // converte no negocio de json sla prr
        .then(data => {
            console.log('resposta do backend:', data);
        })
        .catch(error => {
            console.error('erro:', error);

        });
});
