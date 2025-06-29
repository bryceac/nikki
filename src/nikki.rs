use clap::Parser;

use crate::SubCommand;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>")]
pub struct Nikki {
    #[clap(subcommand)]
    pub subcommand: SubCommand
}