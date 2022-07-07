use iceity::args::Args;
use iceity::progress::{ProgressDialog, ProgressArgs};

use std::process;

use clap::{Parser, CommandFactory};
use iced::{Application, Settings};

macro_rules! match_help_and_exit {
	($args:expr, $method:ident, $struct:ident) => {
		if $args.helps.$method {
			let mut pargs = $struct::command();
			pargs.print_help().unwrap();
			process::exit(0);
		}
	}
}


fn main() -> iced::Result {
    let args = Args::parse();
    match_help_and_exit!(args, help_progress, ProgressArgs);

    let settings = Settings::with_flags(args);
    ProgressDialog::run(settings)
}
