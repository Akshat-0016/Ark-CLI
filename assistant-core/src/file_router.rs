use crate::file_index::FileProfile;

pub fn pick_best_file<'a>(
    query: &str,
    index: &'a std::collections::HashMap<String, FileProfile>
) -> Option<&'a FileProfile> {

    let query = query.to_lowercase();
    let mut best_score = 0;
    let mut best_file = None;

    for (name, profile) in index.iter() {
        let mut score = 0;

        if name.contains("book") && query.contains("chapter") {
            score += 5;
        }
        if name.contains("syllabus") && query.contains("syllabus") {
            score += 5;
        }

        for word in query.split_whitespace() {
            if profile.preview.to_lowercase().contains(word) {
                score += 1;
            }
        }

        if score > best_score {
            best_score = score;
            best_file = Some(profile);
        }
    }

    best_file
}
