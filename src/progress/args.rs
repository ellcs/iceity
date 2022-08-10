use clap::Parser;
use crate::shared::args::GeneralArgs;

#[derive(Parser, Debug, Default)]
//#[clap(override_usage("iceity --progress [OPTIONS]"))]
pub struct ProgressArgs {
    #[clap(flatten)]
    pub general_args: GeneralArgs,

    /// Set the dialog text.
    #[clap(long)]
    pub text: Option<String>,

    /// Set initial percentage
    #[clap(long)]
    pub percentage: Option<f32>,                           

    /// Pulsate progress bar
    /// Not implemented
    #[clap(long)]
    pub pulsate: bool,

    /// Dismiss the dialog when 100% has been reached
    #[clap(long)]
    pub auto_close: bool,

    /// Not implemented: Kill parent process if Cancel button is pressed
    #[clap(long)]
    pub auto_kill: bool,

    /// Hide Cancel button
    #[clap(long)]
    pub no_cancel: bool,

    /// Estimate when progress will reach 100%
    /// Not implemented
    #[clap(long)]
    pub time_remaining: bool
}


#[test]
fn verifys_progress_args() {
    use clap::CommandFactory;
    ProgressArgs::command().debug_assert()
}
