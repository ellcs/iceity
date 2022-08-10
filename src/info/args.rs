use clap::Parser;

#[derive(Parser, Debug, Default)]
//#[clap(override_usage("iceity --progress [OPTIONS]"))]
pub struct InfoArgs {

  /// Set the dialog text
  text: Option<String>,

  /// Set the dialog icon
  icon_name: Option<PathBuf>,

  /// Do not enable text wrapping
  no_wrap: bool,

  /// Do not enable Pango markup
  no_markup: bool,

  /// Enable ellipsizing in the dialog text. This fixes the high window size with long texts
  ellipsize: bool
}
