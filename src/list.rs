use clap::Parser;

#[derive(Parser)]
pub struct List {
    #[clap(default_value = "~/.journal")]
    pub journal: String
}