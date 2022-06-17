use iceity::progress::ProgressDialog;
use iceity::messages::Message;
use iceity::args::Args;

use clap::Parser;
//pub mod progress;
use iced::{Element, Sandbox, Settings};


pub struct IceityApplication {
    arguments: Args,
    state: ChosenWindow
}

pub enum ChosenWindow {
    Progress(ProgressDialog)
}

impl Sandbox for IceityApplication {
    type Message = Message;

    fn new() -> Self {
	    let args = Args::parse();
        Self {
            arguments: args,
            state: ChosenWindow::Progress(ProgressDialog::default())
        }
    }

    fn title(&self) -> String {
        String::from("A simple Progressbar")
    }

    fn update(&mut self, message: Message) {
        match &mut self.state {
            ChosenWindow::Progress(dialog) => {
                match message {
                    Message::Progress(dialog_message) => dialog.update(dialog_message),
                }
            }
                
        }
    }

    fn view(&mut self) -> Element<Message> {
        match &mut self.state {
            ChosenWindow::Progress(dialog) => dialog.view()
        }
    }
}

fn main() -> iced::Result {
    IceityApplication::run(Settings::default())
}
