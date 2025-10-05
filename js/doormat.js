const logoutLink = document.getElementById('logout-link');

if (logoutLink) {
    logoutLink.addEventListener('click', function(event) {
        event.preventDefault();
        axios.post('/api/logout', {}).then(response => {
            window.location.href = response.data;
        });
    });
}

function submitForm() {
    // Get register form data
    const form_data = {
        username : document.getElementsByName('username')[0].value,
        email : document.getElementsByName('email')[0].value,
        password : document.getElementsByName('password')[0].value,
        confirm_password : document.getElementsByName('confirm_password')[0].value,
    };

    axios.post('/api/signup', form_data)
        .then(async response => {
            const responseData = response.data;

            await Swal.fire({
                title: responseData.title,
                text: responseData.message,
                icon: responseData.icon,
            });

            // If the user is successfully signed up, redirect to the login page
            if (response.data.signed_up) {
                window.location.href = "/login";
            }
        })
}
