use chrono::{ LocalResult::Single, prelude::* };
use std::{ ffi::OsStr, fs::File, io, path::Path, io::Read, fmt };
use crate::shared::*;

pub struct Entry {
    pub date: DateTime<Local>,
    pub content: String
}

impl Entry {
    pub fn new() -> Self {
        Self {
            date: Local::now(),
            content: String::default()
        }
    }

    pub fn from(date: DateTime<Local>, content: &str) -> Self {
        Self {
            date,
            content: content.to_owned()
        }
    }

    pub fn from_file(p: &Path) -> Option<Self> {
        if let Some(entry_date) = date_time_from_file(p) {
            if let Ok(content) = contents_from_file(p) {
                Some(Self::from(entry_date, &content))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}\r\n-----\r\n\r\n{}", self.date.format("%Y-%m-%d %R"), self.content)
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date &&
        self.content == other.content
    }
}

fn date_time_from_file(p: &Path) -> Option<DateTime<Local>> {
    if let Some(file_name) = p.file_stem().and_then(OsStr::to_str) {
        convert_string_to_date_time(file_name, "%Y-%m-%d_%H-%M")
    } else {
        None
    }
}

fn contents_from_file(p: &Path) -> Result<String, io::Error> {
    let mut content = String::default();

    File::open(p)?.read_to_string(&mut content)?;

    Ok(content)
}