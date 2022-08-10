use clap::Parser;
use crate::progress::args::ProgressArgs;
use crate::entry::EntryArgs;

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
pub enum ChosenWindow {
    #[clap(name = "--calendar")]
    Calendar,

    #[clap(name = "--entry")]
    Entry(EntryArgs),

    #[clap(name = "--error")]
    Error,

    #[clap(name = "--info")]
    Info,

    #[clap(name = "--file-selection")]
    FileSelection,

    #[clap(name = "--list")]
    List,

    #[clap(name = "--notification")]
    Notification,

    #[clap(name = "--progress")]
    Progress(ProgressArgs),

    #[clap(name = "--question")]
    Question,

    #[clap(name = "--warning")]
    Warning,

    #[clap(name = "--scale")]
    Scale,

    #[clap(name = "--text-info")]
    TextInfo,

    #[clap(name = "--color-selection")]
    ColorSelection,

    #[clap(name = "--password")]
    Password,

    #[clap(name = "--form")]
    Forms,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(flatten)]
    pub helps: Helps,

    #[clap(subcommand)]
    pub chosen_window: ChosenWindow,

    #[clap(long)]
    pub display: Option<String>,
}

#[test]
fn verify_args() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}

