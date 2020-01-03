import { start_app, handle_language_selector_click, set_language, set_page } from 'nemidoonam';

(() => {
    const app = start_app();
    document.getElementById("component-styles").innerHTML = app.css;
    document.getElementById("container").innerHTML = app.html;
})();

// Note should be handled as part of nemidoonam.package.json in /pkg, however I cant yet find a way to get wasm-pack to do this for me
window.nemidoonam = {
    handle_language_selector_click: () => {
        handle_language_selector_click();
        // this should only set the relevant elements, and be done a little less explicitly perhaps
        document.getElementById("container").innerHTML = start_app().html;
    },
    set_language: (id) => {
        set_language(id);
        // this should only set the relevant elements, and be done a little less explicitly perhaps
        document.getElementById("container").innerHTML = start_app().html;
    },
    set_page: (id) => {
        set_page(id);
        // this should only set the relevant elements, and be done a little less explicitly perhaps
        document.getElementById("container").innerHTML = start_app().html;
    },
};
