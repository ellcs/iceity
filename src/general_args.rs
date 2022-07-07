use clap::Parser;

#[derive(Parser, Debug)]
pub struct GeneralArgs {
  //--title=TITEL                                       Den Dialogtitel festlegen
	#[clap(long)]
	pub title: Option<String>,
  //--window-icon=SYMBOLPFAD                            Das Fenstersymbol festlegen
	#[clap(long)]
	pub windiw_icon: Option<String>,
  //--width=BREITE                                      Die Breite festlegen
	#[clap(long)]
	pub width: Option<u32>,
  //--height=HÖHE                                       Die Höhe festlegen
	#[clap(long)]
	pub height: Option<u32>,
  //--timeout=WARTEZEIT                                 Die Wartezeit des Dialogs in Sekunden festlegen
	#[clap(long)]
	pub timeout: Option<u32>,
  //--ok-label=TEXT                                     Legt die Beschriftung des OK-Knopfes fest
	#[clap(long)]
	pub ok_label: Option<String>,
  //--cancel-label=TEXT                                 Die Beschriftung des Abbrechen-Knopfes festlegen
	#[clap(long)]
	pub cancel_label: Option<String>,
  //--extra-button=TEXT                                 Einen zusätzlichen Knopf hinzufügen
	#[clap(long)]
	pub extra_button: Option<String>,
  //--modal                                             Den modalen Hinweis festlegen
	#[clap(long)]
	pub modal: bool,
  //--attach=FENSTER                                    Das übergeordnete Fenster zum Anhängen festlegen
	#[clap(long)]
	pub attach: Option<u32>,
}

#[test]
fn verifys_general_args() {
    use clap::CommandFactory;
    GeneralArgs::command().debug_assert()
}
