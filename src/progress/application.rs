use iced_native::subscription;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::time;

use super::messages::ProgressMessage;
use super::args::ProgressArgs;

use std::str;
use iced::{Row, Column, Element, ProgressBar, Text, Button, button};
use iced::{executor, alignment, Length, Command, Application};

use log::{debug, error};


//#[derive(Default)]
pub struct ProgressDialog {
    pub args: ProgressArgs,
    pub time_elapsed: time::Instant,

    pub value: f32,
    pub ok: button::State,
    pub abort: button::State,
}


enum StdinReaderState {
    Start,
    Reading(BufReader<tokio::io::Stdin>),
    Done
}


impl Application for ProgressDialog {
    type Executor = executor::Default;
    type Message = ProgressMessage;
    type Flags = ProgressArgs;

    fn new(flags: Self::Flags) -> (Self, Command<ProgressMessage>) {
        let start_value = flags.percentage.unwrap_or(0_f32);
        debug!("Starting progress dialog");
        (Self {
            args: flags,
            value: start_value,
            time_elapsed: time::Instant::now(),

            ok: Default::default(),
            abort: Default::default()
        }, Command::none())
    }

    fn title(&self) -> String {
        let default = String::from("Progress");
        self.args.general_args.title.as_ref().unwrap_or(&default).to_string()
    }

    fn update(&mut self, message: ProgressMessage) -> Command<ProgressMessage> {
        match message {
            ProgressMessage::SetProgress(x) => {
                self.value = x;
                if self.args.auto_close && self.value == 100_f32 {
                    std::process::exit(0);
                }
            },
            ProgressMessage::Confirm => {
                std::process::exit(0);
            },
            ProgressMessage::Abort => {
                std::process::exit(1);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<ProgressMessage> {
        let mut ok = Button::new(&mut self.ok, Text::new("Ok")).width(Length::FillPortion(1));
        if self.value == 100_f32 {
            ok = ok.on_press(ProgressMessage::Confirm)
        };

        let mut button_row = Row::new();
        if !self.args.no_cancel {
            button_row = button_row.push({ 
                let mut abort = Button::new(&mut self.abort, Text::new("Abort")).width(Length::FillPortion(1));
                if self.value != 100_f32 {
                    abort = abort.on_press(ProgressMessage::Abort);
                }
                abort
            });
        };
        button_row = button_row.push(ok);

        let view = Column::new()
            .push(Text::new(self.args.text.as_ref().unwrap_or(&String::from(""))))
            .padding(20)
            .push(
                Row::new()
                    .push(
                        ProgressBar::new(0.0..=100.0, self.value)
                    ).push(
                        Text::new(self.value.to_string())
                            .size(30)
                            .width(Length::Units(45))
                            .vertical_alignment(alignment::Vertical::Bottom)
                            .horizontal_alignment(alignment::Horizontal::Right)
                    )
            )
            .padding(20)
            .push(button_row)
            .push({
                let now = self.time_elapsed.elapsed().as_secs();

                let mut time_remaining = String::from("");
                if self.value > 0_f32 && self.value < 100_f32 {
                    let mut remaining = (100 * now) / self.value as u64;
                    remaining -= now;

                    let seconds = remaining % 60;
                    let minutes = remaining / 60;
                    time_remaining = format!("Remaining time: {}:{}", minutes, seconds);
                }

                Text::new(time_remaining)
                    .size(20)
            });

        view.into()
    }

    fn subscription(&self) -> iced::Subscription<ProgressMessage> {
        struct WorkerId;
        let actor = |state| async move {
            match state {
                StdinReaderState::Start => {
                    let reader = BufReader::new(tokio::io::stdin());
                    (None, StdinReaderState::Reading(reader)) 
                },
                StdinReaderState::Reading(mut reader) => {
                    let mut buffer = Vec::new();
                    let res = reader.read_until(b'\n', &mut buffer).await;
                    match res {
                        Ok(0) => {
                            debug!("Stdin has been closed (EOF)");
                            (Some(ProgressMessage::SetProgress(100.0)), StdinReaderState::Done) 
                        },
                        Ok(_n) => {
                            let n = parse_progress_number(&buffer);
                            debug!("Number parsed: {}", n);
                            if n < 100_f32 {
                                (Some(ProgressMessage::SetProgress(n)), StdinReaderState::Reading(reader)) 
                            } else {
                                (Some(ProgressMessage::SetProgress(100_f32)), StdinReaderState::Done) 
                            }
                        },
                        Err(_) => {
                            error!("An error occured while reading from stdin");
                            // await all pending futures (for ever)
                            iced::futures::future::pending().await
                        }
                    }
                },
                StdinReaderState::Done => { 
                    // when the stream reader is closed, we arrived at a dead
                    // end...
                    iced::futures::future::pending().await
                }
            }
        };

        subscription::unfold(std::any::TypeId::of::<WorkerId>(), 
                             StdinReaderState::Start,
                             actor)
    }

}

fn parse_progress_number(buffer : &[u8]) -> f32 {
    let s = str::from_utf8(buffer).map(|p| p.strip_suffix('\n').unwrap_or("")).unwrap_or("");
    s.parse::<f32>().unwrap_or(0_f32)
}

#[test]
fn test_parse_progress_number() {
    assert_eq!(100_f32, parse_progress_number(&Vec::from("100\n")));
    assert_eq!(30.0, parse_progress_number(&Vec::from("30.0\n")));
    assert_eq!(30.5, parse_progress_number(&Vec::from("30.5\n")));
    assert_eq!(25_f32, parse_progress_number(&Vec::from("25\n")));
    assert_eq!(0_f32, parse_progress_number(&Vec::from("0\n")));
    assert_eq!(0_f32, parse_progress_number(&Vec::from("a\n")));
    assert_eq!(0_f32, parse_progress_number(&Vec::from("\n")));
    assert_eq!(0_f32, parse_progress_number(&Vec::from("")));
}
