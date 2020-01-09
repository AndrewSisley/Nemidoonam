use super::label_definition::LabelDefinition;

pub struct Language {
    pub id: i32,
    pub label_definition: LabelDefinition<'static>,
}

impl<'a> Language {
    pub fn get_display_text(&self) -> &'a str {
        return self.label_definition.get_display_text();
    }
}