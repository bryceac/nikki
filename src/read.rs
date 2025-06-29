use std::{ path::Path};

use clap::Parser;

use crate::{shared::{convert_human_date_time_to_computer, convert_string_to_date_time, real_path}, Entry};

use chrono::Datelike;

#[derive(Parser)]
#[clap(version = "0.1.0", about = "display an entry recognized by Nikki. The entries are called by entering a date time in the YYYY-MM-DD HH:MM format like this nikki read -e \"2032-03-03 13:00\".")]
pub struct Read {
    #[clap(default_value = "~/.journal")]
    pub journal: String,

    #[clap(long, short)]
    pub entry: String
}

impl Read {
    pub fn run(&self) {
        let computer_date_time_string = convert_human_date_time_to_computer(&self.entry).expect("Wrong format! the expected format is YYYY-MM-DD HH:MM");

        if let Some(entry_date_time) = convert_string_to_date_time(&self.entry, "%Y-%m-%d %R") {
            let year = format!("{}", entry_date_time.year());
            let month = format!("{}", entry_date_time.month());
            let file_name = format!("{}.md", computer_date_time_string);

            let entry_path = Path::new(&real_path(&self.journal)).join(year).join(month).join(file_name);

            if let Some(entry) = Entry::from_file(&entry_path) {
                println!("{}", entry);
            }
        }
    }
}