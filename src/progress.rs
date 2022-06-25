use crate::messages::Message;
use iced_native::subscription;
use tokio::io::{BufReader, AsyncBufReadExt};

use std::str;
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

enum StdinReaderState {
    Start,
    Reading(BufReader<tokio::io::Stdin>),
    Done
}


impl ProgressDialog {
    pub fn update(&mut self, message: ProgressMessage) {
        match message {
            ProgressMessage::SetProgress(x) => {
                self.value = x
            },

            ProgressMessage::Confirm => {
                std::process::exit(0);
            },

            ProgressMessage::Abort => {
                std::process::exit(1);
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let ok = if self.value == 100_f32 {
            Button::new(&mut self.ok, Text::new("Ok")).on_press(Message::Progress(ProgressMessage::Confirm))
        } else {
            Button::new(&mut self.ok, Text::new("Ok"))
        };
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
                    .spacing(20)
                    .push({ 
                        let abort = Button::new(&mut self.abort, Text::new("Abort"));
                        abort.on_press(Message::Progress(ProgressMessage::Abort))
                    })
                    .push(ok)
            )
            .into()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        struct WorkerId;
        let actor = |state| async move {
            match state {
                StdinReaderState::Start => {
                    let reader = BufReader::new(tokio::io::stdin());
                    //let buffer = Vec::new();
                    (Some(ProgressMessage::SetProgress(0_f32)), StdinReaderState::Reading(reader)) 
                },
                StdinReaderState::Reading(mut reader) => {
                    let mut buffer = Vec::new();
                    println!("here");
                    let res = reader.read_until(b'\n', &mut buffer).await;
                    println!("there");
                    match res {
                        Ok(0) => {
                            // eof
                            unimplemented!();
                        },
                        Ok(_n) => {
                            let n = Self::parse_progress_number(&buffer);
                            println!("read number: {}", n);
                            (Some(ProgressMessage::SetProgress(n as f32)), StdinReaderState::Reading(reader)) 
                        }
                        _ => unimplemented!()
                    }
                },
                StdinReaderState::Done => unimplemented!()

            }
        };
        subscription::unfold(std::any::TypeId::of::<WorkerId>(), 
                             StdinReaderState::Start,
                             actor).map(Message::Progress)
    }

    fn parse_progress_number(buffer : &Vec<u8>) -> f32 {
        println!("buffer content: {:?}", buffer);
        let s = str::from_utf8(&buffer).map(|p| p.strip_suffix('\n').unwrap() ).unwrap_or("");
        println!("buffer s: {:?}", s);
        s.parse::<f32>().unwrap_or(0_f32)
    }
}

#[test]
fn test_parse_progress_number() {
    assert_eq!(100_f32, ProgressDialog::parse_progress_number(&Vec::from("100\n")));
    assert_eq!(30.0, ProgressDialog::parse_progress_number(&Vec::from("30.0\n")));
    assert_eq!(30.5, ProgressDialog::parse_progress_number(&Vec::from("30.5\n")));
    assert_eq!(25_f32, ProgressDialog::parse_progress_number(&Vec::from("25\n")));
    assert_eq!(0_f32, ProgressDialog::parse_progress_number(&Vec::from("0\n")));
    assert_eq!(0_f32, ProgressDialog::parse_progress_number(&Vec::from("a\n")));
}

