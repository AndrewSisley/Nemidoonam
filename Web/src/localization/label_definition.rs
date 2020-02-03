use super::label::Label;
use crate::repos::{ display_language, target_language };

pub struct LabelDefinition<'a> {
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

impl<'a> LabelDefinition<'a> {
    pub fn get_target_text(&self) -> &'a str {
        return match target_language::get() {
            1 => self.english.display_text,
            2 => self.farsi.display_text,
            3 => self.swedish.display_text,
            _ => &"",
        }
    }
}
