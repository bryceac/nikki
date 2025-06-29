use std::{ fs::{self, File }, io::{ self, Write }, path::Path };
use clap::Parser;
use crate::{Entry, shared::real_path};
use chrono::{ prelude::*, Datelike };
use edit::edit_file;

#[derive(Parser)]
#[clap(version = "0.1.0", about = "create journal entries", author = "Bryce Campbell <tonyhawk2100@gmail.com>", long_about = "create journal entries. Typically, this will bring up your default editor, but by using nikki pen -c Message, you can skip all of that.")]
pub struct Pen {

    #[clap(default_value = "~/.journal")]
    pub journal: String,

    #[clap(long, short)]
    pub content: Option<String>
}

impl Pen {
    pub fn run(&self) {
        let now = Local::now();

        let file_name = format!("{}.md", now.format("%Y-%m-%-d_%H-%M"));

        let year = format!("{}", now.year());
        let month = format!("{}", now.month());

        let destination = Path::new(&real_path(&self.journal)).join(year).join(month).join(file_name);

        let _ = fs::create_dir_all(destination.parent().unwrap());

        if let Some(content) = self.content.clone() {
            let entry = Entry::from(now, &content);

            let _ = write_to_file(destination.to_str().unwrap(), &entry);
        } else {
            match edit_file(destination) {
                Ok(_) => {},
                Err(_) => ()
            }
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