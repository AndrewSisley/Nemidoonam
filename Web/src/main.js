import { start_app } from 'nemidoonam';

(() => {
    document.getElementById("container").innerHTML = start_app().html;
})();