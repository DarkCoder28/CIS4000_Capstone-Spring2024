async function register () {
    let username = document.getElementById('username').value;
    if (username === "") {
        errorModal("Registration Error", "Please enter a username");
        return;
    }

    fetch('/api/auth/register_start/' + encodeURIComponent(username), {
        method: 'POST'
    })
    .then(response => response.json() )
    .then(credentialCreationOptions => {
        credentialCreationOptions.publicKey.challenge = Base64.toUint8Array(credentialCreationOptions.publicKey.challenge);
        credentialCreationOptions.publicKey.user.id = Base64.toUint8Array(credentialCreationOptions.publicKey.user.id);
        credentialCreationOptions.publicKey.excludeCredentials?.forEach(function (listItem) {
            listItem.id = Base64.toUint8Array(listItem.id)
        });

        return navigator.credentials.create({
            publicKey: credentialCreationOptions.publicKey
        });
    })
    .then((credential) => {
        fetch('/api/auth/register_finish', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                id: credential.id,
                rawId: Base64.fromUint8Array(new Uint8Array(credential.rawId), true),
                type: credential.type,
                response: {
                    attestationObject: Base64.fromUint8Array(new Uint8Array(credential.response.attestationObject), true),
                    clientDataJSON: Base64.fromUint8Array(new Uint8Array(credential.response.clientDataJSON), true),
                },
            })
        })
        .then((response) => {
            if (response.ok){
                errorModal("Registration Success", "Successfully registered!", "btn-primary");
            } else {
                errorModal("Registration Error", "Error whilst registering");
            }
        });
    })
}

async function login() {
    let username = document.getElementById('username').value;
    if (username === "") {
        errorModal("Login Error", "Please enter a username");
        return;
    }

    fetch('/api/auth/login_start/' + encodeURIComponent(username), {
        method: 'POST'
    })
    .then(response => response.json())
    .then((credentialRequestOptions) => {
        credentialRequestOptions.publicKey.challenge = Base64.toUint8Array(credentialRequestOptions.publicKey.challenge);
        credentialRequestOptions.publicKey.allowCredentials?.forEach(function (listItem) {
            listItem.id = Base64.toUint8Array(listItem.id)
        });

        return navigator.credentials.get({
            publicKey: credentialRequestOptions.publicKey
        });
    })
    .then((assertion) => {
        fetch('/api/auth/login_finish', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                id: assertion.id,
                rawId: Base64.fromUint8Array(new Uint8Array(assertion.rawId), true),
                type: assertion.type,
                response: {
                    authenticatorData: Base64.fromUint8Array(new Uint8Array(assertion.response.authenticatorData), true),
                    clientDataJSON: Base64.fromUint8Array(new Uint8Array(assertion.response.clientDataJSON), true),
                    signature: Base64.fromUint8Array(new Uint8Array(assertion.response.signature), true),
                    userHandle: Base64.fromUint8Array(new Uint8Array(assertion.response.userHandle), true)
                },
            }),
        })
        .then((response) => {
            if (response.ok){
                errorModal("Login Success", "Successfully logged in! Redirecting in 1...", "btn-primary");
                setTimeout(()=>{location.href='/'}, 1000);
            } else {
                errorModal("Login Error", "Error whilst logging in!");
            }
        });
    });
}

async function update_uploader() {
    let new_email = document.getElementById("uploader-email").value;
    fetch("/api/auth/update_email", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            new_email,
        }),
    }).then((response) => {
        if (response.ok) {
            errorModal(
                "Update Success",
                "Successfully updated email! Reloading in 1...",
                "btn-primary"
            );
            setTimeout(() => {
                location.reload();
            }, 1000);
        } else {
            errorModal("Login Error", "Error whilst updating email in!");
        }
    });
}

async function logout() {
    fetch('/api/auth/logout', {
        method: 'POST'
    }).then((_) => {
        location.reload();
    });
}

let login_btn = document.getElementById("login");
let register_btn = document.getElementById("register");
let update_uploader_btn = document.getElementById("update-uploader");
if (login_btn != null && login_btn != undefined) {
    login_btn.addEventListener("click", login);
}
if (register_btn != null && register_btn != undefined) {
    register_btn.addEventListener("click", register);
}
if (update_uploader_btn != null && update_uploader_btn != undefined) {
    update_uploader_btn.addEventListener("click", update_uploader);
}

// Error Modal
async function errorModal(title, error, ok_colour_class = "btn-danger") {
    let container = document.createElement("div");
    let close = () => {container.remove();}
    container.id = "modal"
    let modal = document.createElement("div");
    modal.classList.add("modal", "modal-sheet", "position-static", "d-block", "p-4", "py-md-5");
    modal.tabIndex = -1;
    modal.role = "dialog";
    let dialog = document.createElement("div");
    dialog.classList.add("modal-dialog");
    dialog.role = "document";
    let content = document.createElement("div");
    content.classList.add("modal-content", "rounded-4", "shadow");
    let header = document.createElement("div");
    header.classList.add("modal-header", "border-bottom-0");
    let h1 = document.createElement("h1");
    h1.classList.add("modal-title", "fs-5");
    h1.innerText = title;
    header.appendChild(h1);
    let header_button = document.createElement("button");
    header_button.type = "button";
    header_button.classList.add("btn-close");
    header_button.setAttribute("data-bs-dismiss", "modal");
    header_button.setAttribute("aria-label", "Close");
    header_button.addEventListener("click", close);
    header.appendChild(header_button);
    content.appendChild(header);
    let body = document.createElement("div");
    body.classList.add("modal-body", "py-0");
    let body_content = document.createElement("p");
    body_content.innerText = error;
    body.appendChild(body_content);
    content.appendChild(body);
    let footer = document.createElement("div");
    footer.classList.add("modal-footer", "flex-column", "align-items-stretch", "w-100", "gap-2", "pb-3", "border-top-0");
    let close_btn = document.createElement("button");
    close_btn.type = "button";
    close_btn.classList.add("btn", "btn-lg", ok_colour_class);
    close_btn.setAttribute("data-bs-dismiss", "modal");
    close_btn.addEventListener("click", close);
    close_btn.innerText = "Close";
    footer.appendChild(close_btn);
    content.appendChild(footer);
    dialog.appendChild(content);
    modal.appendChild(dialog);
    container.appendChild(modal);
    document.documentElement.lastElementChild.appendChild(container);
}