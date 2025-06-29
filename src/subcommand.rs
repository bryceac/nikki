use clap::Parser;
use crate::{ List, Pen };

#[derive(Parser)]
pub enum SubCommand {
    #[clap(version = "0.1", author = "Bryce Campbell")]
    List(List),
    #[clap(version = "0.1", author = "Bryce Campbell")]
    Pen(Pen),
}