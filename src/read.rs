use clap::Parser;

use crate::shared::convert_human_date_time_to_computer;

#[derive(Parser)]
pub struct Read {
    #[clap(default_value = "~/.journal")]
    pub journal: String,
    pub entry: String
}

impl Read {
    pub fn run(&self) {
        if let Some(entry_date_time) = convert_human_date_time_to_computer(&self.entry) {

        }
    }
}