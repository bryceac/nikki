use clap::Parser;
use crate::Pen;

#[derive(Parser)]
pub enum SubCommand {
    #[clap(version = "0.1", author = "Bryce Campbell")]
    Pen(Pen)
}