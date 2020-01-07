use super::label::Label;
use crate::repos::display_language;

pub struct LabelDefinition<'a> {
    pub id: i32,
    pub english: Label<'a>,
    pub farsi: Label<'a>,
    pub swedish: Label<'a>,
}

impl<'a> LabelDefinition<'a> {
    pub fn get_display_text(&self) -> &'a str {
        return match display_language::get() {
            1 => self.english.display_text,
            2 => self.farsi.display_text,
            3 => self.swedish.display_text,
            _ => &"",
        }
    }
}
