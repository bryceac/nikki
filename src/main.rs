mod shared;
mod entry;
mod nikki;
mod subcommand;
mod pen;
mod list;

use nikki::Nikki;
use list::List as List;
use subcommand::SubCommand as SubCommand;
use pen::Pen as Pen;
use entry::Entry as Entry;
use clap::Parser;

fn main() {
    let journal = Nikki::parse();

    match journal.subcommand {
        SubCommand::List(list) => list.run(),
        SubCommand::Pen(pen) => pen.run()
    }
}
