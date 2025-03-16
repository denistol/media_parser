use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MediaItem {
    pub name: String,
    pub url: String,
    pub container_selector: String,
    pub item_selector: String,
    pub title_selector: String,
}

pub fn get_media_list() -> Result<Vec<MediaItem>, ()> {
    let result = read_to_string("./media_list.json");

    match result {
        Ok(s) => {
            let deserialized: Vec<MediaItem> =
                serde_json::from_str(&s).expect("Failed to deserialize JSON");
            return Ok(deserialized);
        }
        _ => {
            return Err(());
        }
    }
}
