pub struct SentenceSplitter;

impl SentenceSplitter {
    pub fn split(text: &str) -> Vec<&str> {
        let mut sentences = Vec::new();

        let mut start = 0;

        for (i, ch) in text.char_indices() {
            if matches!(ch, '.' | '!' | '?') {
                let sentence = text[start..=i].trim();

                if !sentence.is_empty() {
                    sentences.push(sentence);
                }

                start = i + ch.len_utf8();
            }
        }

        if start < text.len() {
            let sentence = text[start..].trim();

            if !sentence.is_empty() {
                sentences.push(sentence);
            }
        }

        sentences
    }

    pub fn chunk(text: &str, max_chars: usize) -> Vec<String> {
        let sentences = Self::split(text);

        let mut chunks = Vec::new();
        let mut current = String::new();

        for sentence in sentences {
            let additional = if current.is_empty() {
                sentence.len()
            } else {
                sentence.len() + 1
            };

            if current.len() + additional > max_chars && !current.is_empty() {
                chunks.push(current);
                current = String::new();
            }

            if !current.is_empty() {
                current.push(' ');
            }

            current.push_str(sentence);
        }

        if !current.is_empty() {
            chunks.push(current);
        }

        chunks
    }
}
