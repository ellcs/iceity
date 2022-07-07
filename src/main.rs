use iceity::args::{Args, ChosenWindow};
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

    match args.chosen_window {
        ChosenWindow::Progress(progress_args) => {
            let settings = Settings::with_flags(progress_args);
            ProgressDialog::run(settings)
        },
        _ => panic!()
    }
}
