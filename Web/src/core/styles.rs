use crate::repos::{ text_direction };

pub fn get_shared_css() -> String {
    let display_text_direction = text_direction::get_display();
    let target_text_direction = text_direction::get_target();

    format!(
        "body {{
        direction: {display_text_direction}
        }}
        h1 {{
        font-size: 3em;
        }}
        h2 {{
        font-size: 2.5em;
        }}
        .display-text {{
        direction: {display_text_direction}
        }}
        .target-text {{
        direction: {target_text_direction}
        }}",
        display_text_direction = display_text_direction,
        target_text_direction = target_text_direction,
    )
}
