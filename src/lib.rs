use std::collections::HashSet;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Reject,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Stats {
    pub allowed: usize,
    pub rejected: usize,
}

#[derive(Debug, Default)]
pub struct Filter {
    seen: HashSet<String>,
    stats: Stats,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check(&mut self, message: &str) -> Decision {
        let normalized = normalize(message);

        if self.seen.insert(normalized) {
            self.stats.allowed += 1;
            Decision::Allow
        } else {
            self.stats.rejected += 1;
            Decision::Reject
        }
    }

    pub fn stats(&self) -> Stats {
        self.stats
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
