let search = false;

[...document.getElementsByClassName("toggleSearchClicker")].forEach(element => {
    element.addEventListener("click", () => {
        element.textContent = search?"Search":"Stop Searching";
        document.getElementById("SearchTerm").value = "";
        updateSearch("");
        search = !search;
        updateIndexClasses();
    });
});

document.getElementById("search_form").addEventListener('submit', () => {
    updateSearch(document.getElementById("SearchTerm").value);
});

function updateSearch(search) {
    [...document.getElementsByClassName("recipes_search")].forEach(e => e.innerHTML = "");
    if (search == undefined || search == null || search === "") {
        [...document.getElementsByClassName("recipes")].forEach(e => e.classList.remove("idx_hide"));
        [...document.getElementsByClassName("recipes_search")].forEach(e => e.classList.add("idx_hide"));
        return;
    }
    // Retrieve Recipes
    fetch("/api/search/"+encodeURI(search))
        .then(resp => resp.json())
        .then(resp => {
            // Add recipes to 'recipes_search element
            resp.forEach((recipe) => {
                let tags = "";
                recipe.Tags.forEach(tag => {
                    tags += tag + ", "
                });
                tags = tags.substring(0, tags.length-2);
                let template = `
                <div class="recipe d-flex flex-row pt-4 pb-4 px-2">
                    <div class="img-container d-flex flex-column justify-content-center">
                        <img role="img" class="img idx_tmb" src="${(recipe.CoverImage != undefined && recipe.CoverImage != ""?'/api/images/'+recipe.CoverImage:"/static/images/favicon.png")}" />
                    </div>
                    <div style="width: 1rem;"></div>
                    <div class="d-flex flex-column">
                        <h3><a href="/recipe/${recipe.RecipeID}">${recipe.Name}</a></h3>
                        <p>${recipe.Description}</p>
                        ${recipe.Tags.length > 0?'<p class="mb-0">Tags: '+tags:""}
                    </div>
                </div>
                `;
                [...document.getElementsByClassName("recipes_search")].forEach(e => e.innerHTML += template);
            });
        })
        .then(() => {
            // Swap elements
            [...document.getElementsByClassName("recipes")].forEach(e => e.classList.add("idx_hide"));
            [...document.getElementsByClassName("recipes_search")].forEach(e => e.classList.remove("idx_hide"));
        })
}

function updateIndexClasses() {
    [...document.getElementsByClassName("@shouldshowsearch")].forEach(element => {
        if (search) {
            element.classList.remove("nodisplay");
        } else {
            element.classList.add("nodisplay");
        }
    });
    [...document.getElementsByClassName("@shouldnotshowsearch")].forEach(element => {
        if (!search) {
            element.classList.remove("nodisplay");
        } else {
            element.classList.add("nodisplay");
        }
    });
}
updateIndexClasses();


// Function to set the width of elements with class 'ar_top-row' to be the same as 'main'
function setTopRowWidth() {
    const mainElement = document.querySelector('main');
    const topRowElements = document.querySelectorAll('.idx_top-row');

    topRowElements.forEach(element => {
        element.style.width = `${mainElement.clientWidth}px`;
    });
}
window.addEventListener('DOMContentLoaded', setTopRowWidth);
window.addEventListener('resize', setTopRowWidth);