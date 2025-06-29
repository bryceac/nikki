use clap::Parser;
use crate::{ List, Pen, Read };

#[derive(Parser)]
pub enum SubCommand {
    List(List),
    Pen(Pen),
    Read(Read)
}