use walkdir::WalkDir;
use anyhow::Result;
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct Document {
    pub name: String,
    pub text: String,
}

static INFO_DUMP: Lazy<Vec<Document>> = Lazy::new(|| {
    load_info_dump().unwrap_or_default()
});

pub fn get_info_dump() -> &'static Vec<Document> {
    &INFO_DUMP
}

fn load_info_dump() -> Result<Vec<Document>> {
    let mut docs = vec![];

    for entry in WalkDir::new("info-dump") {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        if entry.file_type().is_file() {
            let path = entry.path();
            let name = path.to_string_lossy().to_string();

            if name.ends_with(".txt") || name.ends_with(".md") {
                if let Ok(text) = std::fs::read_to_string(path) {
                    docs.push(Document { name, text });
                }
            }
        }
    }

    Ok(docs)
}
