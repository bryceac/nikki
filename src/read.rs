use std::{ path::Path};

use clap::Parser;

use crate::{shared::{convert_human_date_time_to_computer, convert_string_to_date_time, real_path}, entry::Entry};

use chrono::Datelike;

#[derive(Parser)]
pub struct Read {
    #[clap(default_value = "~/.journal")]
    pub journal: String,
    pub entry: String
}

impl Read {
    pub fn run(&self) {
        if let Some(entry_date_time_string) = convert_human_date_time_to_computer(&self.entry) {
            if let Some(entry_date_time) = convert_string_to_date_time(&entry_date_time_string, "%Y-%-%m-%d_%H-%M") {
                let year = format!("{}", entry_date_time.year());
                let month = format!("{}", entry_date_time.month());
                let file_name = format!("{}.md", entry_date_time_string);

                let entry_path = Path::new(&real_path(&self.journal)).join(&year).join(month).join(&year).join(&file_name);

                if let Some(entry) = Entry::from_file(&entry_path) {
                    println!("{}", entry);
                }
            }
        }
    }
}