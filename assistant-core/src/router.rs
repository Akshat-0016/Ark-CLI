use crate::memory::Memory;
use crate::skills::system;
use crate::config::load_config;
use crate::cache::{get_cached, set_cached};

use crate::pdf::read_pdf;
use crate::file_index::build_file_index;
use crate::file_router::pick_best_file;

use tokio::process::Command;
use tokio::io::AsyncReadExt;

pub async fn route(input: &str, memory: &mut Memory) -> String {
    memory.remember(input);

    if input.starts_with("system") {
        return system::handle().await;
    }

    // check cache
    if let Some(answer) = get_cached(input) {
        return answer;
    }

    let index = build_file_index();

    let looks_like_file_query =
        input.contains("chapter")
        || input.contains("problem")
        || input.contains("exercise")
        || input.contains("module")
        || input.ends_with(".pdf")
        || input.contains("summari")
        || input.contains("syllabus")
        || input.contains("notes");

    if looks_like_file_query {
        if let Some(profile) = pick_best_file(input, &index) {
            if let Some(pdf_text) = read_pdf(&profile.path) {
                // Build prompt using ONLY the correct file
                let prompt = format!(
                    "You are Ark, a technical assistant.\n\
Using ONLY the following document, answer the user's question.\n\
File: {}\n\n\
--- DOCUMENT EXCERPT ---\n{}\n\
--- END EXCERPT ---\n\n\
User question: {}\n\
Provide a precise, to-the-point answer.",
                    profile.path,
                    pdf_text,
                    input
                );

                let answer = run_ollama(&prompt).await;

                set_cached(input, answer.clone());
                return answer;
            }
        }
    }

    let answer = run_ollama(input).await;

    set_cached(input, answer.clone());
    answer
}


async fn run_ollama(prompt: &str) -> String {
    let cfg = load_config();

    let mut child = Command::new("ollama")
        .args(["run", &cfg.model, prompt])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn ollama");

    let mut stdout = child.stdout.take().unwrap();
    let mut buffer = Vec::new();

    stdout.read_to_end(&mut buffer).await.unwrap();

    String::from_utf8_lossy(&buffer).to_string()
}
