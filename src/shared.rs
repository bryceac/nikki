use url_path::UrlPath;
use chrono::{LocalResult::Single, prelude::* };

pub fn real_path(p: &str) -> String {
    match p {
        path if path.starts_with("~") => shellexpand::tilde(p).into_owned(),
        _ => UrlPath::new(p).normalize()
    }
}

pub fn convert_string_to_date_time(date_time: &str, format: &str) -> Option<DateTime<Local>> {
    if let Ok(date_time) = NaiveDateTime::parse_from_str(date_time, format) {
        if let Single(date_time) = Local.from_local_datetime(&date_time) {
            Some(date_time)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn convert_computer_date_time_to_human(date_time: &str) -> Option<String> {
    if let Some(date_time) = convert_string_to_date_time(date_time, "%Y-%m-%d_%H-%M") {
        Some(format!("{}", date_time.format("%Y-%m-%d %R")))
    } else {
        None
    }
}

pub fn convert_human_date_time_to_computer(date_time: &str) -> Option<String> {
    if let Some(date_time) = convert_string_to_date_time(date_time, "%Y-%m-%d %R") {
        Some(format!("{}", date_time.format("%Y-%m-%d_%H-%M")))
    } else {
        None
    }
}