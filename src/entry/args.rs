use clap::Parser;
use crate::shared::args::GeneralArgs;

#[derive(Parser, Debug)]
#[clap(override_usage("iceity --entry [OPTIONS]"))]
pub struct EntryArgs {
  #[clap(flatten)]
  pub general_args: GeneralArgs,

  /// Set the dialog text.
  #[clap(long)]
  pub text: Option<String>,

  /// Set the entry text
  #[clap(long)]
  pub entry_text: Option<String>,

  /// Hide the entry text
  #[clap(long)]
  pub hide_text: bool
}
