use clap::Parser;
use walkdir::WalkDir;
use std::path::Path;

#[derive(Parser)]
pub struct List {
    #[clap(default_value = "~/.journal")]
    pub journal: String
}

impl List {
    pub fn run(&self) {
        let journal_path = Path::new(&self.journal);

        for item in WalkDir::new(journal_path) {
            if let Ok(entry) = item {
                if entry.path().is_file() {

                }
            }
        }
    }
}