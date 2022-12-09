use iced::theme::{self, Theme};
use iced::widget::{column, container, radio, row, text, Row};
use iced::{Color, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    Styling::run(Settings::default())
}

#[derive(Debug, Default, Clone)]
pub struct Card {
    first_name: String,
    last_name: String,
    age: i32,
    sex: char,
    description: String,
}

#[derive(Debug, Default)]
struct ListOfCards {
    cards: Vec<Card>,
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

// FIXME: Not taking any arguments intentionally for now, once JSON reading is done
// add arguments.
fn create_list_of_cards() -> ListOfCards {
    let first_names = vec!["Kushashwa", "Mohit", "Yatharth", "Vishwesh"];
    let last_names = vec!["Shrimali", "Wankhade", "Wankhade", "Shrimali"];
    let ages = vec![24, 24, 22, 26];
    let genders = vec!['M', 'M', 'M', 'M'];
    let description = "God Level";

    let mut list_of_cards = ListOfCards::default();
    for (first_name, last_name, age, gender) in
        itertools::izip!(first_names, last_names, ages, genders)
    {
        let card = Card {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
            sex: gender,
            description: description.to_string(),
        };

        list_of_cards.cards.push(card);
    }

    list_of_cards
}

pub fn create_card(card: &Card) -> iced::Element<'static, Message> {
    let container_text = "First Name: ".to_owned()
        + &card.first_name
        + "\nLast Name: "
        + &card.last_name
        + "\nAge: "
        + &card.age.to_string()
        + "\nSex: "
        + &card.sex.to_string()
        + "\nDescription:\n"
        + &card.description;
    container(column![text(container_text)]).into()
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
        let choose_theme = [ThemeType::Dark, ThemeType::Light, ThemeType::Custom]
            .iter()
            .fold(
                row![text("Choose a theme:")].spacing(10),
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

        let content = container(column![choose_theme].spacing(20).padding(20).max_width(600))
            .width(Length::Fill)
            .center_x();

        // Not using the following as I want to have 2 separate containers (header container + card container)
        // container(content, card_container)
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .center_x()
        //     .into()

        let all_cards = create_list_of_cards();

        let mut containers = Vec::new();
        for each_card in all_cards.cards.iter() {
            containers.push(
                column![create_card(each_card)]
                    .spacing(10)
                    .padding(20)
                    .max_width(600),
            );
        }

        let row = Row::with_children(
            all_cards
                .cards
                .iter()
                .map(|each_card| {
                    container(
                        column![create_card(each_card)]
                            .spacing(10)
                            .padding(20)
                            .max_width(600),
                    )
                    .into()
                })
                .collect(),
        );

        column![content, row.spacing(10)].into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
