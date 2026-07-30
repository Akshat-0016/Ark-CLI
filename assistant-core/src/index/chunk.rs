#[derive(Debug, Clone)]
pub struct Chunk {
    id: usize,
    document: usize,
    title: String,
    path: String,
    text: String,
}

impl Chunk {
    pub fn new(id: usize, document: usize, title: String, path: String, text: String) -> Self {
        Self {
            id,
            document,
            title,
            path,
            text,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn document(&self) -> usize {
        self.document
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
