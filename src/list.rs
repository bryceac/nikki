use clap::Parser;

#[derive(Parser)]
pub struct List {
    #[clap(default_value = "~/.journal")]
    pub journal: String
}

impl List {
    pub fn run(&self) {
        todo!();
    }
}