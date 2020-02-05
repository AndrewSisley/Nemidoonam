use crate::repos::{ display_language, target_language, available_languages };

pub const LEFT_TO_RIGHT: &'static str = "ltr";
pub const RIGHT_TO_LEFT: &'static str = "rtl";

pub fn get_display() -> String {
    let available_languages = available_languages::get();
    let display_language_id = display_language::get();

    let text_direction = if available_languages.iter().find(|&l| l.id == display_language_id).unwrap().is_right_to_left {
        RIGHT_TO_LEFT
    } else {
        LEFT_TO_RIGHT
    };

    text_direction.to_string()
}

pub fn get_target() -> String {
    let available_languages = available_languages::get();
    let target_language_id = target_language::get();

    let text_direction = if available_languages.iter().find(|&l| l.id == target_language_id).unwrap().is_right_to_left {
        RIGHT_TO_LEFT
    } else {
        LEFT_TO_RIGHT
    };

    text_direction.to_string()
}
