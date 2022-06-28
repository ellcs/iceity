use iceity::args::Args;
use iceity::progress::{ProgressDialog, ProgressArgs};

use clap::{Parser, CommandFactory};
use iced::{Application, Settings};


fn main() -> iced::Result {
    let args = Args::parse();
    if args.helps.help_progress {
        let mut pargs = ProgressArgs::command();
        pargs.print_help();
        panic!();
    }
    ProgressDialog::run(Settings::default())
}
