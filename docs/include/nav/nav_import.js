$(function () {
    $("#nav-placeholder").load("include/nav/nav.html")
});

let collapsed = true;

function clickNav() {
    collapsed = !collapsed;
    if (collapsed) {
        closeNav();
    } else {
        openNav();
    }
}

function openNav() {
    document.getElementById("nav_bar").style.width = "250px";
    document.getElementById("main").style.marginLeft = "250px";
}

function closeNav() {
    document.getElementById("nav_bar").style.width = "0";
    document.getElementById("main").style.marginLeft = "0";
}

window.onload = function () {
    const toggler = document.getElementsByClassName("caret");
    let i;

    for (i = 0; i < toggler.length; i++) {
        toggler[i].addEventListener("click", function() {
            this.parentElement.querySelector(".nested").classList.toggle("active");
            this.classList.toggle("caret-down");
        });
    }
}