use iced::font::Weight;
use iced::widget::svg::{Handle, Svg};
use iced::widget::text_editor::{Action, Content, Edit};
use iced::widget::{button, column, container, row, text, text_editor};
use iced::{Alignment, Command, Element, Font, Length, Padding, Renderer};
use std::borrow::Cow;
use std::sync::Arc;
use typst_ansi_hl::Highlighter;

const TITLE: &str = "Typst ANSI GUI";

#[derive(Debug, Default)]
struct App {
    code: Content,
    result: Content,
    theme: AppTheme,
}

#[derive(Debug, Clone)]
enum Message {
    /// Received some Typst code in the relevant input field.
    TypstInput(Action),

    /// User pressed the 'Submit' button (highlight the code with ANSI).
    Submit,

    /// Received some action for the result text box.
    ResultAction(Action),

    /// Change between light and dark theme.
    InvertTheme,

    /// User requested to copy the output.
    Copy,
}

const MOON_STARS_ICON: &[u8] = include_bytes!("../assets/moon-stars.svg").as_slice();
const MOON_STARS_FILL_ICON: &[u8] = include_bytes!("../assets/moon-stars-fill.svg").as_slice();

#[derive(Copy, Clone, Debug, Default)]
enum AppTheme {
    Light,

    #[default]
    Dark,
}

impl AppTheme {
    /// Returns the inverse theme.
    fn inv(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    /// Theme icon to display (SVG bytes).
    fn icon(self) -> &'static [u8] {
        match self {
            Self::Light => MOON_STARS_ICON,
            Self::Dark => MOON_STARS_FILL_ICON,
        }
    }

    /// Returns the equivalent [`iced::Theme`].
    fn theme(self) -> iced::Theme {
        match self {
            Self::Light => iced::Theme::Light,
            Self::Dark => iced::Theme::Dark,
        }
    }
}

/// Highlights Typst text as ANSI.
fn highlight_typst_to_ansi(input: &str) -> String {
    Highlighter::default().highlight(input).unwrap_or_default()
}

impl App {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TypstInput(input) => self.code.perform(input),
            Message::Submit => {
                self.result = Content::default();
                self.result.perform(Action::Edit(Edit::Paste(Arc::new(
                    highlight_typst_to_ansi(&self.code.text()),
                ))));
            }
            Message::ResultAction(action) => match action {
                // Don't let user edit the output.
                Action::Edit(_) => {}
                action => self.result.perform(action),
            },
            Message::InvertTheme => {
                self.theme = self.theme.inv();
            }
            Message::Copy => {
                return iced::clipboard::write(self.result.text());
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Message, iced::Theme, Renderer> {
        column([
            // Title and theme button
            {
                let bold_font = Font {
                    weight: Weight::Bold,
                    ..Font::default()
                };
                let theme_icon = Svg::new(Handle::from_memory(Cow::from(self.theme.icon())));
                let theme_button = button(theme_icon).on_press(Message::InvertTheme);

                row([
                    container(theme_button)
                        .width(45.0)
                        .height(72.0)
                        .center_y()
                        .into(),
                    container(text("Typst ANSI GUI").size(32.0).font(bold_font))
                        .center_x()
                        .center_y()
                        .width(Length::Fill)
                        .height(72.0)
                        .into(),
                ])
                .align_items(Alignment::Center)
                .into()
            },
            // Input
            text_editor(&self.code)
                .font(Font::with_name("Fira Mono"))
                .on_action(Message::TypstInput)
                .height(200.0)
                .into(),
            // Middle buttons
            container(
                container(row([
                    container({
                        let highlight_button = button(text("Highlight"));
                        if &self.code.text() == "\n" {
                            // Disable button for empty input
                            highlight_button
                        } else {
                            highlight_button.on_press(Message::Submit)
                        }
                    })
                    .width(Length::Fill)
                    .center_x()
                    .into(),
                    container({
                        let copy_button = button(text("Copy"));
                        if &self.result.text() == "\n" {
                            // Disable button for empty output
                            copy_button
                        } else {
                            copy_button.on_press(Message::Copy)
                        }
                    })
                    .width(Length::Fill)
                    .center_x()
                    .into(),
                ]))
                .max_width(400.0)
                .center_x(),
            )
            .height(100.0)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into(),
            // Output
            text_editor(&self.result)
                .font(Font::with_name("Fira Mono"))
                .on_action(Message::ResultAction)
                .height(200.0)
                .into(),
        ])
        .padding(Padding::from([0.0, 50.0]))
        .into()
    }

    fn theme(&self) -> iced::Theme {
        self.theme.theme()
    }
}

fn main() -> iced::Result {
    iced::program(TITLE, App::update, App::view)
        .font(include_bytes!("../assets/FiraSans-Regular.ttf").as_slice())
        .font(include_bytes!("../assets/FiraSans-Bold.ttf").as_slice())
        .font(include_bytes!("../assets/FiraMono-Regular.ttf").as_slice())
        .default_font(Font::with_name("Fira Sans"))
        .theme(App::theme)
        .run()
}
