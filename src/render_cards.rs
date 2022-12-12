use iced::theme;
use iced::widget::{column, container, image, row, text, Column, Container, Row};
use iced::{Length, Renderer};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

use std::error::Error;

pub const MAX_EXPECTED_ITEMS: usize = 12;
const JSON_FILE_PATH: &str = "list_users.json";

macro_rules! get_struct_names {
    (
        #[derive($($derive_name:ident),*)]
        pub struct $name:ident {
            $($fname:ident : $ftype:ty), *
        }
    ) => {
        #[derive($($derive_name),*)]
        pub struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)), *];
                NAMES
            }

            fn get_field(&self, field_name: &str) -> Option<&Vec<String>> {
                match field_name {
                    $(stringify!($fname) => Some(&self.$fname)),
                    *,
                    &_ => None
                }
            }
        }
    }
}

get_struct_names! {
    #[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
    pub struct YTCreator {
        names: Vec<String>,
        avatar_links: Vec<String>,
        descriptions: Vec<String>,
        is_live_status: Vec<String>,
        subscribers: Vec<String>
    }
}

impl YTCreator {
    fn size(&self) -> usize {
        // println!("Field names: {:?}", YTCreator::field_names());
        let mut lengths_all: Vec<usize> = vec![];
        let mut msges: String = "".to_string();
        for field_name in YTCreator::field_names().iter() {
            let len_field: usize = self.get_field(field_name).unwrap().len();
            lengths_all.push(len_field);
            if len_field > MAX_EXPECTED_ITEMS {
                let msg = format!(
                    "Found: {} but got {} for the given field_name: {}\n",
                    len_field, MAX_EXPECTED_ITEMS, field_name
                );
                msges += &msg;
            };
        }

        if !msges.is_empty() {
            panic!("Found more items than expected. {msges}");
        }
        assert!(lengths_all
            .windows(2)
            .all(|single_len| single_len[0] == single_len[1]));
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

pub fn create_row(
    cards: &ListOfCards,
    img_handles_row: &[image::Handle],
    offset: usize,
) -> Row<'static, Message> {
    Row::with_children(
        cards
            .cards
            .iter()
            .enumerate()
            .map(|(idx, each_card)| {
                container(
                    row![
                        column![create_card(each_card)].spacing(50).padding(20),
                        column![profile_pic(
                            130,
                            img_handles_row.get(offset + idx).unwrap().to_owned()
                        )]
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

pub fn get_json_data(json_path: Option<&str>) -> YTCreator {
    let json_file_path = json_path.unwrap_or(JSON_FILE_PATH);
    let obj = read_json(json_file_path).unwrap();
    obj
}

pub fn get_all_avatars(json_obj: &YTCreator) -> Vec<image::Handle> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_json_data() {
        let expected_output: YTCreator = YTCreator {
            names: vec!["Kush".to_string(), "Kushashwa".to_string()],
            avatar_links: vec!["https://avatars.githubusercontent.com/u/19997320?v=4".to_string(), "https://media-exp1.licdn.com/dms/image/C4D03AQGiAbH1TT3fNA/profile-displayphoto-shrink_800_800/0/1642226109876?e=2147483647&v=beta&t=fcJojobq-NZv0oNX_WW9RrCsYsoTqz0TSYMcC6zOGco".to_string()],
            descriptions: vec!["Developer".to_string(), "Developer".to_string()],
            is_live_status: vec!["true".to_string(), "true".to_string()],
            subscribers: vec!["100".to_string(), "200".to_string()]
        };
        assert_eq!(get_json_data(Some("test_assets/sample_data.json")), expected_output);
    }
}
