use chrono::prelude::*;

pub struct Entry {
    pub date: DateTime<Utc>,
    pub content: String
}

impl Entry {
    pub fn new() -> Self {
        Self {
            date: Utc::now(),
            content: String::default()
        }
    }
}