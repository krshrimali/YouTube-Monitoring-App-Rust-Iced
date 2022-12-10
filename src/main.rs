use iced::theme::{self, Theme};
use iced::widget::{column, container, horizontal_rule, image, radio, row, text, Container, Row};
use iced::{Color, Length, Renderer, Sandbox, Settings};

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (1600, 800);
    Styling::run(settings)
}

#[derive(Debug, Default, Clone)]
pub struct Card {
    first_name: String,
    last_name: String,
    age: i32,
    sex: char,
    description: String,
}

#[derive(Debug, Default, Clone)]
pub struct ListOfCards {
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
fn create_list_of_cards() -> Vec<ListOfCards> {
    let first_names = vec![
        "Kushashwa",
        "Mohit",
        "Yatharth",
        "Vishwesh",
        "Random",
        "Second",
        "Know",
        "One",
        "First",
        "Second",
        "Third",
        "Fourth",
    ];
    let last_names = vec![
        "Shrimali",
        "Wankhade",
        "Wankhade",
        "Shrimali",
        "Random",
        "Random",
        "More",
        "More",
        "First Name",
        "Second Name",
        "Third Name",
        "Fourth Name",
    ];
    let ages = vec![24, 24, 22, 26, 22, 23, 28, 30, 28, 30, 23, 24];
    let genders = vec!['M', 'M', 'M', 'M', 'F', 'F', 'M', 'F', 'M', 'F', 'F', 'F'];
    let description = "God Level";

    let mut list_of_cards = vec![ListOfCards::default()];
    for (count_so_far, (first_name, last_name, age, gender)) in
        itertools::izip!(first_names, last_names, ages, genders).enumerate()
    {
        let card = Card {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
            sex: gender,
            description: description.to_string(),
        };

        if count_so_far % 4 != 0 || count_so_far == 0 {
            list_of_cards.last_mut().unwrap().cards.push(card);
        } else {
            list_of_cards.push(ListOfCards::default());
            list_of_cards.last_mut().unwrap().cards.push(card);
        }
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

pub fn create_row(cards: &ListOfCards) -> Row<'static, Message> {
    Row::with_children(
        cards
            .cards
            .iter()
            .map(|each_card| {
                container(
                    row![
                        column![create_card(each_card)]
                            .width(Length::Fill)
                            .spacing(50)
                            .padding(20),
                        column![profile_pic(130)]
                            .width(Length::Units(130))
                            .height(Length::Units(150))
                            .padding(20)
                    ]
                    .height(Length::Fill),
                )
                .center_y()
                .width(Length::Fill)
                .style(theme::Container::Box)
                .into()
            })
            .collect(),
    )
}

fn profile_pic<'a>(width: u16) -> Container<'a, Message> {
    container(
        // This should go away once we unify resource loading on native
        // platforms
        if cfg!(target_arch = "wasm32") {
            image("profile_images/Kush.png")
        } else {
            image(format!(
                "{}/profile_images/Kush.png",
                env!("CARGO_MANIFEST_DIR")
            ))
        }
        .height(Length::Units(width))
        .width(Length::Units(width)),
    )
    .width(Length::Fill)
    .center_x()
}

impl Sandbox for Styling {
    type Message = Message;

    fn new() -> Styling {
        // Because dark as default is cool :D
        Styling { theme: Theme::Dark }
    }

    fn title(&self) -> String {
        String::from("YT Monitoring App (by KRS)")
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
                |column: iced_native::widget::row::Row<'_, Message, Renderer>, theme| {
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

        let footer = container(
            column![text(
                "Thank you for being here, this was an app by Kushashwa Ravi Shrimali"
            )]
            .spacing(20)
            .padding(20)
            .max_width(600),
        )
        .width(Length::Fill)
        .center_x()
        .center_y();

        let all_cards = create_list_of_cards();
        let binding = ListOfCards::default();
        let first_row_cards = all_cards.get(0).unwrap_or(&binding);
        let second_row_cards = all_cards.get(1).unwrap_or(&binding);
        let third_row_cards = all_cards.get(2).unwrap_or(&binding);

        container(column![
            content,
            create_row(first_row_cards),
            create_row(second_row_cards),
            create_row(third_row_cards),
            horizontal_rule(100),
            footer,
        ])
        .height(Length::Shrink)
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
