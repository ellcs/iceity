use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::time;

use super::args::EntryArgs;
use super::messages::EntryMessage;


use iced::{Row, Column, Element, ProgressBar, Text, Button, button};
use iced::{executor, alignment, Length, Command, Application};


pub struct EntryDialog {
    pub args: EntryArgs,

    pub ok: button::State,
    pub abort: button::State
}

impl Application for EntryDialog {
    type Executor = executor::Default;
    type Message = EntryMessage;
    type Flags = EntryArgs;

    fn new(flags: Self::Flags) -> (Self, Command<EntryMessage>) {
        (Self {
            args: flags,

            ok: Default::default(),
            abort: Default::default(),
        }, Command::none())
    }

    fn title(&self) -> String {
        let default = String::from("Entry");
        self.args.general_args.title.as_ref().unwrap_or(&default).to_string()
    }

    fn update(&mut self, message: EntryMessage) -> Command<EntryMessage> {
        match message {
            EntryMessage::Ok => {
                std::process::exit(0);
            },
            EntryMessage::Abort => {
                std::process::exit(1);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<EntryMessage> {
        let default = &String::from("");
        let view = Column::new()
            .padding(20)
            .push({
                let text_content = self.args.text.as_ref().unwrap_or(default);
                Text::new(text_content)
            })
            .push({
                let ok_text = Text::new("Ok");
                Button::new(&mut self.ok, ok_text)
                    .width(Length::FillPortion(1))
                    .on_press(EntryMessage::Ok)
            });
        view.into()
    }

}

