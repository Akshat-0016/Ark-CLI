pub struct Memory {
    pub history: Vec<String>,
    pub max: usize,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            history: Vec::with_capacity(32),
            max: 100, 
        }
    }

    pub fn remember(&mut self, line: &str) {
        if self.history.len() >= self.max {
            self.history.remove(0);
        }
        self.history.push(line.to_owned());
    }

    pub fn last(&self) -> Option<&String> {
        self.history.last()
    }
}
