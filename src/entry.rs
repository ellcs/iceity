use clap::Parser;

#[derive(Parser, Debug)]
#[clap(override_usage("iceity --entry [OPTIONS]"))]
pub struct EntryArgs {
    /// Set the dialog text.
    /// Not implemented
    #[clap(long, requires("entry"))]
    pub text: Option<String>,
}
