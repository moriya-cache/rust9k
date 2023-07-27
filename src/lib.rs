use std::collections::HashSet;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Reject,
}

#[derive(Debug, Default)]
pub struct Filter {
    seen: HashSet<String>,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check(&mut self, message: &str) -> Decision {
        let normalized = normalize(message);

        if self.seen.insert(normalized) {
            Decision::Allow
        } else {
            Decision::Reject
        }
    }
}

fn normalize(message: &str) -> String {
    message
        .nfkc()
        .flat_map(char::to_lowercase)
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
