//! PDF-specific text normalization.

pub struct PdfNormalizer;

impl PdfNormalizer {
    /// Normalize raw PDF text into readable paragraphs.
    pub fn normalize(text: &str) -> String {
        let filtered = Self::filter_lines(text);
        let repaired = Self::repair_paragraphs(filtered);
        Self::compress_whitespace(&repaired)
    }

    /// Remove obvious PDF artifacts.
    fn filter_lines(text: &str) -> Vec<String> {
        text.lines()
            .map(str::trim)
            .filter(|line| Self::keep(line))
            .map(str::to_owned)
            .collect()
    }

    /// Merge wrapped lines while preserving paragraphs.
    fn repair_paragraphs(lines: Vec<String>) -> String {
        let mut out = String::new();

        for (i, line) in lines.iter().enumerate() {
            out.push_str(line);

            let next = lines.get(i + 1);

            if let Some(next) = next {
                if Self::ends_paragraph(line) {
                    out.push_str("\n\n");
                } else if Self::starts_new_section(next) {
                    out.push_str("\n\n");
                } else {
                    out.push(' ');
                }
            }
        }

        out
    }

    /// Remove excessive whitespace.
    fn compress_whitespace(text: &str) -> String {
        let mut out = String::new();
        let mut blank = false;

        for line in text.lines() {
            let line = line.trim();

            if line.is_empty() {
                if !blank {
                    out.push('\n');
                    blank = true;
                }
            } else {
                if !out.is_empty() && !out.ends_with('\n') {
                    out.push('\n');
                }

                out.push_str(line);
                blank = false;
            }
        }

        out
    }

    fn keep(line: &str) -> bool {
        let line = line.trim();

        if line.is_empty() {
            return false;
        }

        // Standalone page numbers.
        if line.parse::<usize>().is_ok() {
            return false;
        }

        // "261 of 963"
        if let Some((a, b)) = line.split_once(" of ") {
            if a.trim().parse::<usize>().is_ok() && b.trim().parse::<usize>().is_ok() {
                return false;
            }
        }

        // URLs
        if line.starts_with("http://") || line.starts_with("https://") || line.starts_with("www.") {
            return false;
        }

        // Running headers.
        const HEADERS: &[&str] = &[
            "Page ",
            "Figure ",
            "Listing ",
            "Table ",
            "Chapter ",
            "Copyright",
            "The Rust Programming Language",
        ];

        if HEADERS
            .iter()
            .any(|h| line.starts_with(h) || line.contains(h))
        {
            return false;
        }

        // Simple timestamps.
        if line.contains(" AM") || line.contains(" PM") {
            return false;
        }

        true
    }

    fn ends_paragraph(line: &str) -> bool {
        matches!(
            line.chars().last(),
            Some('.') | Some('!') | Some('?') | Some(':')
        )
    }

    fn starts_new_section(line: &str) -> bool {
        if line.is_empty() {
            return false;
        }

        // "1 Introduction"
        if line
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            return true;
        }

        // ALL CAPS headings.
        if line.len() > 3 && line.chars().all(|c| c.is_uppercase() || c.is_whitespace()) {
            return true;
        }

        false
    }
}
