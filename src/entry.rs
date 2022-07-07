use clap::Parser;

#[derive(Parser, Debug)]
#[clap(override_usage("iceity --entry [OPTIONS]"))]
pub struct EntryArgs {
    /// Set the dialog text.
    /// Not implemented
    #[clap(long)]
    pub text: Option<String>,
}

#[test]
fn verify_entry_args() {
    use clap::CommandFactory;
    EntryArgs::command().debug_assert()
}
