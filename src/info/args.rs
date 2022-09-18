use clap::Parser;
use std::path::PathBuf;
use crate::shared::args::GeneralArgs;

#[derive(Parser, Debug, Default)]
//#[clap(override_usage("iceity --progress [OPTIONS]"))]
pub struct InfoArgs {
  #[clap(flatten)]
  pub general_args: GeneralArgs,

  /// Set the dialog text
  #[clap(long)]
  pub text: Option<String>,

  /// Set the dialog icon. 
  #[clap(long)]
  pub icon_name: Option<String>,

  /// Set the dialog icon. 
  #[clap(long)]
  pub icon_path: Option<PathBuf>,

  /// Do not enable text wrapping
  #[clap(long)]
  pub no_wrap: bool,

  /// Do not enable Pango markup
  #[clap(long)]
  pub no_markup: bool,

  /// Enable ellipsizing in the dialog text. This fixes the high window size with long texts
  #[clap(long)]
  pub ellipsize: bool
}
