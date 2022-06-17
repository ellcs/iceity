use crate::messages::Message;

use iced::{Row, Alignment, Column, Element, ProgressBar, Text, Button, button};

#[derive(Default)]
pub struct ProgressDialog {
    pub value: f32,
    pub ok: button::State,
    pub abort: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum ProgressMessage {
    Confirm,
    Abort,
    SetProgress(f32)
}

impl ProgressDialog {
    pub fn update(&mut self, message: ProgressMessage) {
        match message {
            ProgressMessage::SetProgress(x) => {
                self.value = x
            },

            ProgressMessage::Confirm => {
                panic!()
            },
            ProgressMessage::Abort => {
                panic!()
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(
                Row::new()
                    .push(
                        ProgressBar::new(0.0..=100.0, self.value)
                    ).push(
                        Text::new(format!("{}", self.value))
                    )
            )
            .padding(20)
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.abort, Text::new("Abort")).on_press(Message::Progress(ProgressMessage::Abort))
                    )
                    .align_items(Alignment::End)
                    .push(
                        Button::new(&mut self.ok, Text::new("Ok")).on_press(Message::Progress(ProgressMessage::Confirm))
                    )
            )
            .into()
    }

    // https://docs.rs/iced_native/latest/iced_native/subscription/fn.unfold.html
    pub fn fetch_stdin() {

    }
}
