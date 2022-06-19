use iceity::args::Args;
use iceity::messages::Message;
use iceity::progress::ProgressDialog;

use clap::Parser;
use iced::{Element, executor, Command, Application, Subscription, Settings};


pub struct IceityApplication {
    arguments: Args,
    state: ChosenWindow
}

pub enum ChosenWindow {
    Progress(ProgressDialog)
}

impl Application for IceityApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
	    let args = Args::parse();
        (Self {
            arguments: args,
            state: ChosenWindow::Progress(ProgressDialog::default())
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("A simple Progressbar")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match &mut self.state {
            ChosenWindow::Progress(dialog) => {
                match message {
                    Message::Progress(dialog_message) => dialog.update(dialog_message),
                }
            }
                
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match &self.state {
            ChosenWindow::Progress(dialog) => {
				dialog.subscription()
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
