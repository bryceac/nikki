use clap::Parser;

use crate::SubCommand;

#[derive(Parser)]
#[clap(version = "0.1.0", about = "CLI based journal system.", author = "Bryce Campbell <tonyhawk2100@gmail.com>", long_about = "A CLI based journaling system that makes journaling easier for those that want the most flexibility.")]
pub struct Nikki {
    #[clap(subcommand)]
    pub subcommand: SubCommand
}