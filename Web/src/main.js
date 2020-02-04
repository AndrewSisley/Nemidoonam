import {
    start_app,
    handle_display_language_selector_click,
    handle_target_language_selector_click,
    set_display_language,
    set_target_language,
    reverse_languages,
    set_page
} from 'nemidoonam';

(() => {
    const app = start_app();
    document.getElementById("component-styles").innerHTML = app.css;
    document.getElementById("container").innerHTML = app.html;
})();

// Note should be handled as part of nemidoonam.package.json in /pkg, however I cant yet find a way to get wasm-pack to do this for me
window.nemidoonam = {
    handle_display_language_selector_click: () => {
        handle_display_language_selector_click();
        // this should only set the relevant elements, and be done a little less explicitly perhaps
        document.getElementById("container").innerHTML = start_app().html;
    },
    handle_target_language_selector_click: () => {
        handle_target_language_selector_click();
        // this should only set the relevant elements, and be done a little less explicitly perhaps
        document.getElementById("container").innerHTML = start_app().html;
    },
    set_display_language: (id) => {
        set_display_language(id);
        // this should only set the relevant elements, and be done a little less explicitly perhaps
        document.getElementById("container").innerHTML = start_app().html;
    },
    set_target_language: (id) => {
        set_target_language(id);
        // this should only set the relevant elements, and be done a little less explicitly perhaps
        document.getElementById("container").innerHTML = start_app().html;
    },
    reverse_languages: () => {
        reverse_languages();
        // this should only set the relevant elements, and be done a little less explicitly perhaps
        document.getElementById("container").innerHTML = start_app().html;
    },
    set_page: (id) => {
        set_page(id);
        // this should only set the relevant elements, and be done a little less explicitly perhaps
        const app = start_app();
        document.getElementById("component-styles").innerHTML = app.css;
        document.getElementById("container").innerHTML = app.html;
    },
};
