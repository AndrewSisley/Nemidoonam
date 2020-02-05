use super::label_definition::LabelDefinition;

pub struct Language {
    pub id: i32,
    pub is_right_to_left: bool,
    pub label_definition: LabelDefinition<'static>,
}

impl<'a> Language {
    pub fn get_display_text(&self) -> &'a str {
        return self.label_definition.get_display_text();
    }
}