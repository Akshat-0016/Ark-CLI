#[derive(Debug, Clone)]
pub struct Document {
    id: usize,
    title: String,
    path: String,
    text: String,
}

impl Document {
    pub fn new(id: usize, title: String, path: String, text: String) -> Self {
        Self {
            id,
            title,
            path,
            text,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
