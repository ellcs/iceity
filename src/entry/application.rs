use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::time;

use super::args::EntryArgs;
use super::messages::EntryMessage;


use iced::{Row, Column, Element, ProgressBar, Text, Button, button};
use iced::{executor, alignment, Length, Command, Application};
use iced::{text_input, TextInput};


pub struct EntryDialog {
    pub args: EntryArgs,

    pub entry_text: String,
    pub entry_text_state: text_input::State,
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

            entry_text: String::from(""),
            entry_text_state: Default::default(),
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
                println!("{}", self.entry_text);
                std::process::exit(0);
            },
            EntryMessage::Abort => {
                std::process::exit(1);
            }
            EntryMessage::InputChanged(value) => {
                self.entry_text = value;
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
                TextInput::new(
                    &mut self.entry_text_state,
                   "",
                   &self.entry_text,
                   EntryMessage::InputChanged
                )
                .padding(10)
                .size(20)
            })
            .push({
                let abort_text = Text::new("Abort");
                let abort = Button::new(&mut self.abort, abort_text)
                    .width(Length::FillPortion(1))
                    .on_press(EntryMessage::Abort);

                let ok_text = Text::new("Ok");
                let ok = Button::new(&mut self.ok, ok_text)
                    .width(Length::FillPortion(1))
                    .on_press(EntryMessage::Ok);

                Row::new().push(abort).push(ok)
            });
        view.into()
    }

}

