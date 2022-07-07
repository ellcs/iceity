use iced_native::subscription;
use tokio::io::{BufReader, AsyncBufReadExt};

use std::str;
use iced::{Row, Column, Element, ProgressBar, Text, Button, button};
use iced::{executor, Command, Application};

use clap::Parser;

use crate::general_args::GeneralArgs;


#[derive(Parser, Debug)]
//#[clap(override_usage("iceity --progress [OPTIONS]"))]
pub struct ProgressArgs {
	#[clap(flatten)]
	pub general_args: GeneralArgs,

    /// Set the dialog text.
    //#[clap(long, requires("progress"))]
    #[clap(long)]
    pub text: Option<String>,

    /// Set initial percentage
    #[clap(long)]
    pub percentage: Option<f32>,                           

    /// Pulsate progress bar
    /// Not implemented
    #[clap(long)]
    pub pulsate: bool,

    /// Dismiss the dialog when 100% has been reached
    #[clap(long)]
    pub auto_close: bool,

    /// Kill parent process if Cancel button is pressed
    /// Not implemented
    #[clap(long)]
    pub auto_kill: bool,

    /// Hide Cancel button
    #[clap(long)]
    pub no_cancel: bool,

    /// Estimate when progress will reach 100%
    /// Not implemented
    #[clap(long)]
    pub time_remaining: bool
}


#[test]
fn verifys_progress_args() {
    use clap::CommandFactory;
    ProgressArgs::command().debug_assert()
}

//#[derive(Default)]
pub struct ProgressDialog {
    pub args: ProgressArgs,

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
    Reading(BufReader<tokio::io::Stdin>)
}


impl Application for ProgressDialog {
    type Executor = executor::Default;
    type Message = ProgressMessage;
    type Flags = ProgressArgs;

    fn new(flags: Self::Flags) -> (Self, Command<ProgressMessage>) {
        let start_value = flags.percentage.unwrap_or(0_f32);
        (Self {
            args: flags,
            value: start_value,

            ok: Default::default(),
            abort: Default::default()
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Progress")
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
        let ok = if self.value == 100_f32 {
            Button::new(&mut self.ok, Text::new("Ok")).on_press(ProgressMessage::Confirm)
        } else {
            Button::new(&mut self.ok, Text::new("Ok"))
        };

        let view = Column::new()
            .push(Text::new(self.args.text.as_ref().unwrap_or(&String::from(""))))
            .padding(20)
            .push(
                Row::new()
                    .push(
                        ProgressBar::new(0.0..=100.0, self.value)
                    ).push(
                        Text::new(format!("{}", self.value))
                    )
            )
            .padding(20);

        if !self.args.no_cancel {
            view.push(
                Row::new()
                    .spacing(20)
                    .push({ 
                        let abort = Button::new(&mut self.abort, Text::new("Abort"));
                        abort.on_press(ProgressMessage::Abort)
                    })
                    .push(ok)
            ).into()
        } else {
            view.into()
        }
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
                            // await all pending futures (for ever)
                            iced::futures::future::pending().await
                        },
                        Ok(_n) => {
                            let n = parse_progress_number(&buffer);
                            println!("read number: {}", n);
                            (Some(ProgressMessage::SetProgress(n as f32)), StdinReaderState::Reading(reader)) 
                        },
                        Err(_) => {
                            // await all pending futures (for ever)
                            iced::futures::future::pending().await
                        }
                    }
                }
            }
        };
        subscription::unfold(std::any::TypeId::of::<WorkerId>(), 
                             StdinReaderState::Start,
                             actor)
    }

}

fn parse_progress_number(buffer : &Vec<u8>) -> f32 {
    println!("buffer content: {:?}", buffer);
    let s = str::from_utf8(&buffer).map(|p| p.strip_suffix('\n').unwrap_or("")).unwrap_or("");
    println!("buffer s: {:?}", s);
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
