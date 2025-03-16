use crate::NewsItem;
use anyhow::Context;
use std::{
    fs::{self, File},
    io::Read,
};

pub fn get_prev_items(input_dir: &str) -> Result<(Vec<NewsItem>), anyhow::Error> {
    let mut all_items: Vec<NewsItem> = Vec::new();

    let entries = fs::read_dir(input_dir).context("Failed to read directory")?;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let mut file = File::open(&path).context(format!("Failed to open file {:?}", path))?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .context(format!("Failed to read file {:?}", path))?;

            let items: Vec<NewsItem> = serde_json::from_str(&contents)
                .context(format!("Failed to deserialize file {:?}", path))?;
            all_items.extend(items);
        }
    }

    Ok(all_items)
}

pub fn merge_json_files(
    input_dir: &str,
    output_file: &str,
) -> Result<(Vec<NewsItem>), anyhow::Error> {
    let all_items = get_prev_items(input_dir).unwrap();
    let serialized =
        serde_json::to_string_pretty(&all_items).context("Failed to serialize combined data")?;
    fs::write(output_file, serialized).context("Failed to write output file")?;

    Ok(all_items)
}
