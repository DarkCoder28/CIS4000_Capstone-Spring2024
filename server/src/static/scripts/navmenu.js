[...document.getElementsByClassName("nav-link")].forEach(element => {
    let path = element.getAttribute("href");
    if (path == document.location.pathname) {
        element.classList.add("active");
    }
});