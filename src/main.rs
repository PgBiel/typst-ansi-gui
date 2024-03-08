use iced::widget::text_editor::{Action, Content};
use iced::widget::{button, column, container, text, text_editor, vertical_space};
use iced::{Application, Command, Element, Renderer, Settings};

#[derive(Debug, Default)]
struct App {
    code: Content,
}

#[derive(Debug, Clone)]
enum Message {
    /// Received some Typst code in the relevant input field.
    TypstInput(Action),

    /// User pressed the 'Submit' button (highlight the code with ANSI).
    Submit,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Typst ANSI GUI".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TypstInput(input) => self.code.perform(input),
            Message::Submit => println!("Yay!"),
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        column([
            container(text("Typst ANSI GUI")).padding(50.0).into(),
            text_editor(&self.code)
                .height(60.0)
                .on_action(Message::TypstInput)
                .into(),
            vertical_space().height(50.0).into(),
            if self.code.text().is_empty() {
                button(text("Submit"))
            } else {
                button(text("Submit")).on_press(Message::Submit)
            }
            .into(),
        ])
        .into()
    }
}

fn main() {
    App::run(Settings::default()).unwrap()
}
