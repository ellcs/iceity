use clap::Parser;
use crate::progress::{ProgressArgs, EntryArgs};

#[derive(Parser, Debug)]
pub struct Helps {
    #[clap(long, display_order(1))]
    pub help_all: bool,
    #[clap(long, display_order(1))]
    pub help_general: bool,
    #[clap(long, display_order(1))]
    pub help_calendar: bool,
    #[clap(long, display_order(1))]
    pub help_entry: bool,
    #[clap(long, display_order(1))]
    pub help_error: bool,
    #[clap(long, display_order(1))]
    pub help_info: bool,
    #[clap(long, display_order(1))]
    pub help_file_selection: bool,
    #[clap(long, display_order(1))]
    pub help_list: bool,
    #[clap(long, display_order(1))]
    pub help_notification: bool,
    #[clap(long, display_order(1))]
    pub help_progress: bool,
    #[clap(long, display_order(1))]
    pub help_question: bool,
    #[clap(long, display_order(1))]
    pub help_warning: bool,
    #[clap(long, display_order(1))]
    pub help_scale: bool,
    #[clap(long, display_order(1))]
    pub help_text_info: bool,
    #[clap(long, display_order(1))]
    pub help_color_selection: bool,
    #[clap(long, display_order(1))]
    pub help_password: bool,
    #[clap(long, display_order(1))]
    pub help_forms: bool,
    #[clap(long, display_order(1))]
    pub help_misc: bool,
    #[clap(long, display_order(1))]
    pub help_gtk: bool
}


#[derive(Parser, Debug)]
pub struct ChosenWindow {
    #[clap(long, conflicts_with_all(&["entry", "error", "info", "file-selection", "list",
    "notification", "progress", "question", "warning", "scale", "text-info", "color-selection",
    "password", "forms"]))]
    pub calendar: bool,

    #[clap(long, conflicts_with_all(&["calendar", "error", "info", "file-selection", "list",
    "notification", "progress", "question", "warning", "scale", "text-info", "color-selection",
    "password", "forms"]))]
    pub entry: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "info", "file-selection", "list",
    "notification", "progress", "question", "warning", "scale", "text-info", "color-selection",
    "password", "forms"]))]
    pub error: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "file-selection", "list",
    "notification", "progress", "question", "warning", "scale", "text-info", "color-selection",
    "password", "forms"]))]
    pub info: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "info", "list", "notification",
    "progress", "question", "warning", "scale", "text-info", "color-selection", "password",
    "forms"]))]
    pub file_selection: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "info", "file-selection",
    "notification", "progress", "question", "warning", "scale", "text-info", "color-selection",
    "password", "forms"]))]
    pub list: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "info", "file-selection",
    "list", "progress", "question", "warning", "scale", "text-info", "color-selection", "password",
    "forms"]))]
    pub notification: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "info", "file-selection",
    "list", "notification", "question", "warning", "scale", "text-info", "color-selection",
    "password", "forms"]))]
    pub progress: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "info", "file-selection",
    "list", "notification", "progress", "warning", "scale", "text-info", "color-selection",
    "password", "forms"]))]
    pub question: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "info", "file-selection",
    "list", "notification", "progress", "question", "scale", "text-info", "color-selection",
    "password", "forms"]))]
    pub warning: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "info", "file-selection",
    "list", "notification", "progress", "question", "warning", "text-info", "color-selection",
    "password", "forms"]))]
    pub scale: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "info", "file-selection",
    "list", "notification", "progress", "question", "warning", "scale", "color-selection",
    "password", "forms"]))]
    pub text_info: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "info", "file-selection",
    "list", "notification", "progress", "question", "warning", "scale", "text-info", "password",
    "forms"]))]
    pub color_selection: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "info", "file-selection",
    "list", "notification", "progress", "question", "warning", "scale", "text-info",
    "color-selection", "forms"]))]
    pub password: bool,

    #[clap(long, conflicts_with_all(&["calendar", "entry", "error", "info", "file-selection",
    "list", "notification", "progress", "question", "warning", "scale", "text-info",
    "color-selection", "password"]))]
    pub forms: bool,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(flatten)]
    pub helps: Helps,

    #[clap(flatten)]
    pub chosen_window: ChosenWindow,

    #[clap(flatten)]
    pub progress_args: ProgressArgs,

    #[clap(flatten)]
    pub entry_args: EntryArgs,

    #[clap(long)]
    pub display: Option<String>,
}

#[test]
fn verify_progress_args() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}

