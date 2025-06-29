use clap::Parser;
use walkdir::WalkDir;
use std::{ ffi::OsStr, path::Path };
use crate::shared::*;

#[derive(Parser)]
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
                println!("item found");
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