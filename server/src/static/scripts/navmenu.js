[...document.getElementsByClassName("nav-link")].forEach(element => {
    let path = element.getAttribute("href");
    if (path == document.location.pathname) {
        element.classList.add("active");
    }
});

let minimizeNavMenu = true;
let collapseNavMenu = true;

[...document.getElementsByClassName("navmenucollapse_toggle")].forEach(element => {
    element.addEventListener("click", () => {
        collapseNavMenu = !collapseNavMenu;
        updateClasses();
    });
});

function updateClasses() {
    [...document.getElementsByClassName("@NavMenuCssClass")].forEach(element => {
        if (collapseNavMenu) {
            element.classList.add("collapse");
        } else {
            element.classList.remove("collapse");
        }
    });
    // private string? MinimizeNavMenuCssClass => minimizeNavMenu ? "hide" : null;
    [...document.getElementsByClassName("@MinimizeNavMenuCssClass")].forEach(element => {
        if (minimizeNavMenu) {
            element.classList.add("hide");
        } else {
            element.classList.remove("hide");
        }
    });
}
updateClasses();