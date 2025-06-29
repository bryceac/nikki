use clap::Parser;

#[derive(Parser)]
pub struct Write {

    #[clap(default_value = "~/.journal")]
    pub journal: String,

    #[clap(long, short)]
    pub content: Option<String>
}