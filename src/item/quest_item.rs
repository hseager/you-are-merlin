use super::{Item, ItemType};

use crate::text_format::TextFormatter;

#[derive(Clone, Debug)]
pub struct QuestItem {
    name: String,
}

impl QuestItem {
    pub fn new(name: String) -> QuestItem {
        QuestItem { name }
    }
}

impl Item for QuestItem {
    fn name(&self) -> &String {
        &self.name
    }

    fn display_name(&self) -> String {
        self.name.text_bold()
    }

    fn item_type(&self) -> ItemType {
        ItemType::QuestItem
    }

    fn display_info(&self) -> String {
        let mut info = String::new();

        info.push_str(&format!(
            "{} - ({})\n",
            self.display_name(),
            &self.item_type().to_string(),
        ));

        format!("\n{}", info.trim())
    }
}
