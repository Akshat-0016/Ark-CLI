use std::collections::HashMap;

use crate::nlp;

use super::intent::Intent;

pub struct IntentClassifier;

impl IntentClassifier {
    pub fn classify(input: &str) -> Intent {
        let scores = Self::scores(input);

        scores
            .into_iter()
            .max_by_key(|(_, score)| *score)
            .map(
                |(intent, score)| {
                    if score == 0 {
                        Intent::Unknown
                    } else {
                        intent
                    }
                },
            )
            .unwrap_or(Intent::Unknown)
    }

    pub fn scores(input: &str) -> HashMap<Intent, usize> {
        let words = nlp::process_words(input).unwrap_or_default();

        let mut scores = HashMap::new();

        scores.insert(Intent::Search, 0);
        scores.insert(Intent::Memory, 0);
        scores.insert(Intent::Bash, 0);
        scores.insert(Intent::Task, 0);
        scores.insert(Intent::Help, 0);

        for word in &words {
            match word.as_str() {
                // Search
                "search" | "find" | "lookup" | "look" | "where" | "what" | "why" => {
                    *scores.entry(Intent::Search).or_default() += 2;
                }

                // Memory
                "remember" | "save" | "store" | "memorize" | "recall" => {
                    *scores.entry(Intent::Memory).or_default() += 3;
                }

                // Tasks
                "task" | "todo" | "deadline" | "remind" => {
                    *scores.entry(Intent::Task).or_default() += 3;
                }

                // Help
                "help" | "usage" | "how" => {
                    *scores.entry(Intent::Help).or_default() += 2;
                }

                _ => {}
            }
        }

        if let Some(first) = words.first() {
            if matches!(
                first.as_str(),
                "ls" | "cd"
                    | "pwd"
                    | "cat"
                    | "mkdir"
                    | "rm"
                    | "cp"
                    | "mv"
                    | "touch"
                    | "grep"
                    | "find"
                    | "cargo"
                    | "git"
            ) {
                *scores.entry(Intent::Bash).or_default() += 10;
            }
        }

        scores
    }
}
