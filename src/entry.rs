use chrono::prelude::*;
use std::{ ffi::OsStr, fs::File, io, path::Path, io::Read, fmt };

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
        if let (Some(date), Ok(content)) = (date_time_from_file(p), contents_from_file(p)) {
            Some(Self::from(date, &content))
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

fn contents_from_file(p: &Path) -> Result<String, io::Error> {
    let mut content = String::default();

    File::open(p)?.read_to_string(&mut content)?;

    Ok(content)
}