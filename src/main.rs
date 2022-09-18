use iceity::args::{Args, ChosenWindow};
use iceity::progress::application::ProgressDialog;
use iceity::progress::args::ProgressArgs;
use iceity::info::args::InfoArgs;
use iceity::info::application::InfoDialog;
use iceity::entry::application::EntryDialog;

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

/// The settings for iced applications have to be set. We the the source
/// from the parsed arguments, which are provided as a flag. 
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
    match_help_and_exit!(args, help_info,     InfoArgs);

    match args.chosen_window {
        ChosenWindow::Progress(progress_args) => {
            let mut settings = Settings::with_flags(progress_args);
            adjust_settings_by_general_args!(settings);
            ProgressDialog::run(Settings::from(settings))
        },
        ChosenWindow::Info(progress_args) => {
            let mut settings = Settings::with_flags(progress_args);
            adjust_settings_by_general_args!(settings);
            InfoDialog::run(Settings::from(settings))
        },
        ChosenWindow::Entry(entry_args) => { 
            let mut settings = Settings::with_flags(entry_args);
            adjust_settings_by_general_args!(settings);
            EntryDialog::run(Settings::from(settings))
        },
        ChosenWindow::Calendar => { panic!() },
        ChosenWindow::Error => { panic!() },
        ChosenWindow::FileSelection => { panic!() },
        ChosenWindow::List => { panic!() },
        ChosenWindow::Notification => { panic!() },
        ChosenWindow::Question => { panic!() },
        ChosenWindow::Warning => { panic!() },
        ChosenWindow::Scale => { panic!() },
        ChosenWindow::TextInfo => { panic!() },
        ChosenWindow::ColorSelection => { panic!() },
        ChosenWindow::Password => { panic!() },
        ChosenWindow::Forms => { panic!() },
    }
}
