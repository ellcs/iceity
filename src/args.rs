use clap::Parser;

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
    #[clap(long)]
    calendar: bool,

    #[clap(long)]
    entry: bool,

    #[clap(long)]
    error: bool,

    #[clap(long)]
    info: bool,

    #[clap(long)]
    file_selection: bool,

    #[clap(long)]
    list: bool,

    #[clap(long)]
    notification: bool,

    #[clap(long)]
    pub progress: bool,

    #[clap(long)]
    question: bool,

    #[clap(long)]
    warning: bool,

    #[clap(long)]
    scale: bool,

    #[clap(long)]
    text_info: bool,

    #[clap(long)]
    color_selection: bool,

    #[clap(long)]
    password: bool,

    #[clap(long)]
    forms: bool,

    #[clap(long)]
    display: Option<String>,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(flatten)]
    pub helps: Helps,

    #[clap(flatten)]
    pub chosen_window: ChosenWindow,
}
