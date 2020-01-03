extern crate wacm;

use wacm::Component;
use crate::pages::{ home, alphabet };

pub fn get_component() -> Component {
    return Component {
        css: ".nav-bar-top {
        display: inline-block;
        background-color: lightgray;
        }
        .nav-bar-top a:hover {
        background-color: grey;
        }
        .nav-bar-top a {
        display: inline-block;
        font-size: 1.3em;
        padding: 5px 20px 5px 20px;
        }".to_string(),
        html: format!(
            "<nav class='nav-bar-top'>
            <a onclick='window.nemidoonam.set_page(1)'>{home_title}</a>
            <a onclick='window.nemidoonam.set_page(2)'>{alphabet_title}</a>
            </nav>",
            home_title = home::TITLE.get_display_text(),
            alphabet_title = alphabet::TITLE.get_display_text()
        ),
    }
}
