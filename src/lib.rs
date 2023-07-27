use std::collections::HashSet;

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
        if self.seen.insert(message.to_owned()) {
            Decision::Allow
        } else {
            Decision::Reject
        }
    }
}
