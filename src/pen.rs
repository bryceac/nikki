use std::{ fs::File, io::{ self, Write }, path::Path };
use clap::Parser;
use crate::{Entry, shared::real_path};
use chrono::prelude::*;

#[derive(Parser)]
pub struct Pen {

    #[clap(default_value = "~/.journal")]
    pub journal: String,

    #[clap(long, short)]
    pub content: Option<String>
}

impl Pen {
    pub fn run(&self) {
        let now = Utc::now();

        if let Some(content) = self.content.clone() {
            let entry = Entry::from(now, &content);

            let file_name = format!("{}.md", now.format("%Y-%m-%-d_%H-%M"));

            let destination = Path::new(real_path(journal)).join(path)

            write_to_file(p, &entry)
        }
    }
}

fn write_to_file(p: &str, entry: &Entry) -> Result<(), io::Error> {
    let mut output = File::create(Path::new(&real_path(p)))?;
    
    match write!(output, "{}", entry.content) {
        Ok(()) => Ok(()),
        Err(error) => Err(error)
    }
}