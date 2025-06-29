use clap::Parser;
use walkdir::WalkDir;
use std::{ ffi::OsStr, path::Path };
use crate::shared::*;

#[derive(Parser)]
#[clap(version = "0.1.0", about = "list entries recognized by nikki", author = "Bryce Campbell <tonyhawk2100@gmail.com>", long_about = "display journal entries recognized by Nikki. They are list as date times in the format of YYYY-MM-DD HH:MM")]
pub struct List {
    #[clap(default_value = "~/.journal")]
    pub journal: String
}

impl List {
    pub fn run(&self) {
        let input = real_path(&self.journal);

        let journal_path = Path::new(&input);

        for item in WalkDir::new(journal_path) {
            if let Ok(entry) = item {
                if entry.path().is_file() {
                    if let Some(file_name) = entry.path().file_stem().and_then(OsStr::to_str) {
                        if let Some(entry_string) = convert_computer_date_time_to_human(file_name) {
                            println!("{}", entry_string)
                        }
                    }    
                }
            }
        }
    }
}