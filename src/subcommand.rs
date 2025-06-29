use clap::Parser;
use crate::{ List, Pen, Read };

#[derive(Parser)]
pub enum SubCommand {
    #[clap(version = "0.1", author = "Bryce Campbell")]
    List(List),
    #[clap(version = "0.1", author = "Bryce Campbell")]
    Pen(Pen),
    #[clap(version = "0.1", author = "Bryce Campbell")]
    Read(Read)
}