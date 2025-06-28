use chrono::prelude::*;
use std::{ ffi::OsStr, fs::File, path::Path };

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

    pub fn from(date: DateTime<Utc>, content: &str) -> Self {
        Self {
            date,
            content: content.to_owned()
        }
    }

    pub fn from_file(p: &Path) -> Option<Self> {
        if let Some(date) = date_time_from_file(p) {

        } else {
            None
        }
    }
}

fn date_time_from_file(p: &Path) -> Option<DateTime<Utc>> {
    if let Some(file) = p.file_stem().and_then(OsStr::to_str) {
        if let Ok(date_time) = NaiveDateTime::parse_from_str(file, "%Y-&m-%d_%H-%M") {
            Some(Utc.from_utc_datetime(&date_time))
        } else {
            None
        }
    } else {
        None
    }
}