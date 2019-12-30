import { start_app } from 'nemidoonam';

(() => {
    const app = start_app();
    document.getElementById("component-styles").innerHTML = app.css;
    document.getElementById("container").innerHTML = app.html;
})();

