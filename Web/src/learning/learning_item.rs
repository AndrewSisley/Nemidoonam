use crate::localization::{ label_definition::LabelDefinition };

pub struct LearningItem {
    pub target_languages: Vec<i32>,
    pub label_definition: LabelDefinition<'static>,
}

impl<'a> LearningItem {
    pub fn get_display_text(&self) -> &'a str {
        return self.label_definition.get_display_text();
    }
}

impl<'a> LearningItem {
    pub fn get_target_text(&self) -> &'a str {
        return self.label_definition.get_target_text();
    }
}
