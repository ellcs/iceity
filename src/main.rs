use clap::Parser;

#[derive(Parser, Debug)]
struct Helps {
    #[clap(long)]
	help_all: bool,
    #[clap(long)]
	help_general: bool,
    #[clap(long)]
	help_calendar: bool,
    #[clap(long)]
	help_entry: bool,
    #[clap(long)]
	help_error: bool,
    #[clap(long)]
	help_info: bool,
    #[clap(long)]
	help_file_selection: bool,
    #[clap(long)]
	help_list: bool,
    #[clap(long)]
	help_notification: bool,
    #[clap(long)]
	help_progress: bool,
    #[clap(long)]
	help_question: bool,
    #[clap(long)]
	help_warning: bool,
    #[clap(long)]
	help_scale: bool,
    #[clap(long)]
	help_text_info: bool,
    #[clap(long)]
	help_color_selection: bool,
    #[clap(long)]
	help_password: bool,
    #[clap(long)]
	help_forms: bool,
    #[clap(long)]
	help_misc: bool,
    #[clap(long)]
	help_gtk: bool
}

#[derive(Parser, Debug)]
struct ChosenWindow {
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
    progress: bool,

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
struct Args {
	#[clap(flatten)]
	helps: Helps,

	#[clap(flatten)]
	chosen_window: ChosenWindow
}


fn main() {
	let args = Args::parse();
    println!("Hello, world!");
}
