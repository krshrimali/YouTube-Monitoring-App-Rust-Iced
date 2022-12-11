use iced::theme;
use iced::widget::{column, container, image, row, text, Column, Container, Row};
use iced::{Length, Renderer};
use std::fs::File;
use serde::Deserialize;
use std::io::BufReader;

use std::error::Error;

const MAX_EXPECTED_ITEMS: usize = 12;
const JSON_FILE_PATH: &str = "list_users.json";

#[derive(Deserialize, Debug, Default, Clone)]
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
pub enum TextType {
    Header,
    Footer,
    Normal,
}

// FIXME: Not taking any arguments intentionally for now, once JSON reading is done
// add arguments.
pub fn create_list_of_cards(obj: &YTCreator) -> Vec<ListOfCards> {
    let mut list_of_cards = vec![ListOfCards::default()];
    for (count_so_far, (name, description, is_live_status, subscribers, avatar_link)) in
        itertools::izip!(
            &obj.names,
            &obj.descriptions,
            &obj.is_live_status,
            &obj.subscribers,
            &obj.avatar_links
        )
        .enumerate()
    {
        let card = Card {
            name: name.to_string(),
            description: description.to_string(),
            is_live_status: is_live_status.to_string(),
            subscribers: subscribers.to_string(),
            avatar_link: avatar_link.to_string(),
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

pub fn create_row(cards: &ListOfCards, img_handles: Vec::<image::Handle>) -> Row<'static, Message> {
    Row::with_children(
        cards
            .cards
            .iter()
            .enumerate()
            .map(|(idx, each_card)| {
                container(
                    row![
                        column![create_card(each_card)].spacing(50).padding(20),
                        column![profile_pic(130, img_handles.get(idx).unwrap().to_owned())]
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

pub fn profile_pic<'a>(width: u16, img_handle: image::Handle) -> Container<'a, Message> {
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
        image(img_handle)
            .height(Length::Units(width))
            .width(Length::Units(width)),
    )
    .width(Length::Fill)
    .center_x()
}

pub fn create_text<'a>(
    input_text: String,
    text_type: TextType,
) -> Container<'a, Message, Renderer> {
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

pub fn get_json_data() -> YTCreator {
    let obj = read_json(JSON_FILE_PATH).unwrap();
    obj
}

pub fn get_all_avatars(json_obj: &YTCreator) -> Vec::<image::Handle> {
    let mut out_handles: Vec<image::Handle> = Vec::new();
    for link in &json_obj.avatar_links {
        let img_obj = reqwest::blocking::get(link).ok();
        let img_bytes = match img_obj {
            Some(bytes) => bytes.bytes().ok(),
            None => None,
        }
        .unwrap();
        let out_img: image::Handle = image::Handle::from_memory(img_bytes.to_vec());
        out_handles.push(out_img);
    }

    out_handles
}
