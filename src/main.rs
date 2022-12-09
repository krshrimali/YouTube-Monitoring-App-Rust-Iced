use iced::theme::{self, Theme};
use iced::widget::{
    column, container, horizontal_rule, radio, row, text, vertical_rule,
};
use iced::{alignment, Element};
use iced::{Color, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    Styling::run(Settings::default())
}

#[derive(Debug)]
struct Card {
    first_name: String,
    last_name: String,
    age: i32,
    sex: char,
    description: String,
}

#[derive(Default)]
struct Styling {
    theme: Theme,
}

#[derive(Debug, PartialEq, Clone, Eq, Copy)]
pub enum ThemeType {
    Light,
    Dark,
    Custom,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(ThemeType),
}

pub fn create_card(
    first_name: String,
    last_name: String,
    age: i32,
    sex: char,
    description: String,
) -> iced::Element<'static, Message> {
    container(column![text(
        "First Name: ".to_owned()
            + &first_name
            + "\nLast Name: "
            + &last_name
            + "\nAge: "
            + &age.to_string()
            + "\nSex: "
            + &sex.to_string()
            + "\nDescription:\n"
            + &description
    )])
    .into()
}

impl Sandbox for Styling {
    type Message = Message;

    fn new() -> Styling {
        // Because dark as default is cool :D
        Styling { theme: Theme::Dark }
    }

    fn title(&self) -> String {
        String::from("Theme chooser (iced)")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = match theme {
                    ThemeType::Dark => Theme::Dark,
                    ThemeType::Light => Theme::Light,
                    ThemeType::Custom => Theme::custom(theme::Palette {
                        background: Color::from_rgb(1.0, 0.9, 1.0),
                        text: Color::BLACK,
                        primary: Color::from_rgb(0.5, 0.5, 0.0),
                        success: Color::from_rgb(0.0, 1.0, 0.0),
                        danger: Color::from_rgb(1.0, 0.0, 0.0),
                    }),
                }
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let choose_theme =
            [ThemeType::Dark, ThemeType::Light, ThemeType::Custom]
                .iter()
                .fold(
                    column![text("Choose a theme:")].spacing(10),
                    |column, theme| {
                        column.push(radio(
                            format!("{:?}", theme),
                            *theme,
                            Some(match self.theme {
                                Theme::Dark => ThemeType::Dark,
                                Theme::Light => ThemeType::Light,
                                Theme::Custom { .. } => ThemeType::Custom,
                            }),
                            Message::ThemeChanged,
                        ))
                    },
                );

        // let card = column![
        //     row![horizontal_rule(18)],
        //     column![text("Kushashwa Ravi Shrimali")
        //         .width(Length::Fill)
        //         .horizontal_alignment(alignment::Horizontal::Center),],
        //     text("Another row")
        //         .width(Length::Fill)
        //         .horizontal_alignment(alignment::Horizontal::Center),
        //     vertical_rule(18),
        // ];

        let card = create_card(
            String::from("Kush"),
            String::from("Shrimali"),
            24,
            'M',
            String::from("I'm a developer..."),
        );
        let content = column![choose_theme, horizontal_rule(38), card]
            .spacing(20)
            .padding(20)
            .max_width(600);
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
