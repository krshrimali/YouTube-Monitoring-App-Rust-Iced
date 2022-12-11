use iced::theme::{self, Theme};
use iced::widget::{
    column, container, horizontal_rule, image, radio, row, text, Column, Container, Row,
};
use iced::{Color, Length, Renderer, Sandbox, Settings};
use std::fs::File;
// mod parser;
use serde::Deserialize;
use std::io::BufReader;

use std::error::Error;

const MAX_EXPECTED_ITEMS: usize = 12;
const JSON_FILE_PATH: &str = "list_users.json";

#[derive(Deserialize, Debug)]
pub struct YTCreator {
    names: Vec<String>,
    avatar_links: Vec<String>,
    descriptions: Vec<String>,
    is_live_status: Vec<String>,
    subscribers: Vec<String>,
}

impl YTCreator {
    fn size(&self) -> usize {
        self.names.len()
    }

    fn slice_to(&self, count_items: usize) -> YTCreator {
        let mut new_obj = YTCreator {
            names: Vec::new(),
            avatar_links: Vec::new(),
            descriptions: Vec::new(),
            is_live_status: Vec::new(),
            subscribers: Vec::new(),
        };
        new_obj.names = self.names.get(0..count_items).unwrap().to_vec();
        new_obj.avatar_links = self.avatar_links.get(0..count_items).unwrap().to_vec();
        new_obj.descriptions = self.descriptions.get(0..count_items).unwrap().to_vec();
        new_obj.is_live_status = self.is_live_status.get(0..count_items).unwrap().to_vec();
        new_obj.subscribers = self.subscribers.get(0..count_items).unwrap().to_vec();
        new_obj
    }
}

// Straight from the documentation
pub fn read_json(file_path: &str) -> Result<YTCreator, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `YTCreator`.
    let u: YTCreator = serde_json::from_reader(reader)?;
    if u.size() > MAX_EXPECTED_ITEMS {
        Ok(u.slice_to(MAX_EXPECTED_ITEMS))
    } else {
        Ok(u)
    }
}

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (1600, 800);
    Styling::run(settings)
}

#[derive(Debug, Default, Clone)]
pub struct Card {
    name: String,
    avatar_link: String,
    description: String,
    is_live_status: String,
    subscribers: String,
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

#[derive(Debug)]
enum TextType {
    Header,
    Footer,
    Normal,
}

// FIXME: Not taking any arguments intentionally for now, once JSON reading is done
// add arguments.
fn create_list_of_cards(obj: YTCreator) -> Vec<ListOfCards> {
    let mut list_of_cards = vec![ListOfCards::default()];
    for (count_so_far, (name, description, is_live_status, subscribers, avatar_link)) in
        itertools::izip!(
            obj.names,
            obj.descriptions,
            obj.is_live_status,
            obj.subscribers,
            obj.avatar_links
        )
        .enumerate()
    {
        let card = Card {
            name,
            description,
            is_live_status,
            subscribers,
            avatar_link,
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
    let container_text = "Name: ".to_owned()
        + &card.name
        + "\nDescription: "
        + &card.description
        + "\nSubscriber Count: "
        + &card.subscribers
        + "\nIs Live?: "
        + &card.is_live_status
        + "\n";
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
                        column![create_card(each_card)].spacing(50).padding(20),
                        column![profile_pic(130, each_card.avatar_link.to_owned())]
                            .width(Length::Units(130))
                            .height(Length::Units(150))
                            .padding(20)
                    ]
                    .align_items(iced::Alignment::End)
                    .height(Length::Fill),
                )
                .width(Length::Fill)
                .center_y()
                .style(theme::Container::Box)
                .into()
            })
            .collect(),
    )
}

fn profile_pic<'a>(width: u16, link: String) -> Container<'a, Message> {
    let img_obj = reqwest::blocking::get(link).ok();
    let img_bytes = match img_obj {
        Some(bytes) => {
            bytes.bytes().ok()
        },
        None => None
    }.unwrap();

    let out_img: image::Handle = image::Handle::from_memory(img_bytes.to_vec());

    container(
        // Keeping this here for the record on how to use image paths
        // if cfg!(target_arch = "wasm32") {
        //     image("path")
        // } else {
        //     image(format!(
        //         "path",
        //         env!("CARGO_MANIFEST_DIR")
        //     ))
        // }
        image(out_img)
        .height(Length::Units(width))
        .width(Length::Units(width)),
    )
    .width(Length::Fill)
    .center_x()
}

fn create_text<'a>(input_text: String, text_type: TextType) -> Container<'a, Message, Renderer> {
    let text_column: Column<'_, Message, Renderer> = column![text(input_text)];
    let text_column_with_props = match text_type {
        TextType::Header => text_column.spacing(20).padding(20).max_width(600),
        TextType::Footer => text_column.spacing(20).padding(20).max_width(600),
        TextType::Normal => text_column,
    };

    container(text_column_with_props)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
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

        let footer = create_text(
            "Thank you for being here, this was an app by Kushashwa Ravi Shrimali".to_string(),
            TextType::Footer,
        );

        let title_header = create_text(
            "Welcome! Here is the status of your favorite YouTubers:".to_string(),
            TextType::Header,
        );

        let obj = read_json(JSON_FILE_PATH).unwrap();
        let all_cards = create_list_of_cards(obj);
        let binding = ListOfCards::default();
        let first_row_cards = all_cards.get(0).unwrap_or(&binding);
        let second_row_cards = all_cards.get(1).unwrap_or(&binding);
        let third_row_cards = all_cards.get(2).unwrap_or(&binding);

        container(column![
            content,
            horizontal_rule(10),
            title_header,
            horizontal_rule(10),
            create_row(first_row_cards),
            create_row(second_row_cards),
            create_row(third_row_cards),
            horizontal_rule(10),
            footer,
            horizontal_rule(10),
        ])
        .height(Length::Shrink)
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
