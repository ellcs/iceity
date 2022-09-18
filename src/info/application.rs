use super::args::InfoArgs;
use super::messages::InfoMessage;


use iced::{Column, Element, Text, Button, button, executor, Length, Command, 
           Application};


pub struct InfoDialog {
    pub args: InfoArgs,

    pub ok: button::State,
}

impl Application for InfoDialog {
    type Executor = executor::Default;
    type Message = InfoMessage;
    type Flags = InfoArgs;

    fn new(flags: Self::Flags) -> (Self, Command<InfoMessage>) {
        (Self {
            args: flags,

            ok: Default::default(),
        }, Command::none())
    }

    fn title(&self) -> String {
        let default = String::from("Information");
        self.args.general_args.title.as_ref().unwrap_or(&default).to_string()
    }

    fn update(&mut self, message: InfoMessage) -> Command<InfoMessage> {
        match message {
            InfoMessage::Ok => {
                std::process::exit(0);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<InfoMessage> {
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
                    .on_press(InfoMessage::Ok)
            });
        view.into()
    }

}
