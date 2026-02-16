use std::collections::HashMap;
use walkdir::WalkDir;
use crate::config::load_config;
use crate::pdf::read_pdf;

pub struct FileProfile {
    pub path: String,
    pub preview: String, 
}

pub fn build_file_index() -> HashMap<String, FileProfile> {
    let mut index = HashMap::new();
    let cfg = load_config();

    for entry in WalkDir::new(&cfg.info_dump_path) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path().to_string_lossy().to_string();

        if !name.ends_with(".pdf") {
            continue;
        }

        if let Some(text) = read_pdf(&path) {
            let preview = text.chars().take(1500).collect::<String>();

            index.insert(
                name.to_lowercase(), 
                FileProfile { path, preview }
            );
        }
    }

    index
}
