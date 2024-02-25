use iced::{Application, Command, Element, executor, Settings, Theme};
use iced::widget::text;

fn main() -> iced::Result {
    IcedApplication::run(Settings::default())
}

struct IcedApplication {

}

#[derive(Debug, Clone)]
enum Message {

}

impl Application for IcedApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {},
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Iced Template")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello, iced!").into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}