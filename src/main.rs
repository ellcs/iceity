use iceity::args::{Args, ChosenWindow};
use iceity::progress::{ProgressDialog, ProgressArgs};

#[macro_use]
extern crate log;

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

macro_rules! adjust_settings_by_general_args {
    ($settings:expr) => {
        if let Some(width) = $settings.flags.general_args.width {
            $settings.window.size = (width, $settings.window.size.1);
        }
        if let Some(height) = $settings.flags.general_args.height {
            $settings.window.size = ($settings.window.size.0, height);
        }
        $settings.window.resizable = false;
    }
}

#[test]
fn test_settings_by_general_args_sets_width_and_height() {
    let args = ProgressArgs::default();
    let mut settings : Settings<ProgressArgs> = Settings::with_flags(args);
    adjust_settings_by_general_args!(settings);
    // default size of settings
    assert_eq!((1024, 768), settings.window.size);

    settings.flags.general_args.width = Some(100);
    settings.flags.general_args.height = Some(100);
    adjust_settings_by_general_args!(settings);
    // default size of settings
    assert_eq!((100, 100), settings.window.size);
}

fn main() -> iced::Result {
    env_logger::init();
    let args = Args::parse();
    match_help_and_exit!(args, help_progress, ProgressArgs);

    match args.chosen_window {
        ChosenWindow::Progress(progress_args) => {
            let mut settings = Settings::with_flags(progress_args);
            adjust_settings_by_general_args!(settings);
            ProgressDialog::run(Settings::from(settings))
        },
        _ => panic!()
    }
}
