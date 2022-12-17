use iced::theme;
// use iced::widget::container::Appearance;
use iced::widget::{column, container, image, row, text, Column, Container, Row};
use iced::{Length, Renderer};
use iced_core::Color;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use std::error::Error;

pub const MAX_EXPECTED_ITEMS: usize = 12;
const JSON_FILE_PATH: &str = "list_users.json";

// This also adds an impl: get_field to get the corresponding field from the field name (&str)
macro_rules! get_struct_names {
    (
        #[derive($($derive_name:ident),*)]
        pub struct $name:ident {
            $(pub $fname:ident : $ftype:ty), *
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

            pub fn get_field(&self, field_name: &str) -> Option<&Vec<String>> {
                match field_name {
                    $(stringify!($fname) => Some(&self.$fname)),
                    *,
                    &_ => None
                }
            }

            fn slice_to(&self, count_items: usize) -> YTCreator {
                YTCreator {
                    $($fname: self.$fname.get(0..count_items).expect(&format!("Not enough elements to be sliced into, maybe check the input {count_items} again.")).to_vec()),
                    *
                }
            }

            fn set_field(&mut self, field_name: &str, field_data: Vec<String>) -> &mut YTCreator {
                match field_name {
                    $(stringify!($fname) => {
                        self.$fname = field_data
                    }),
                    *,
                    &_ => ()
                };
                self
            }
        }
    }
}

get_struct_names! {
    #[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
    pub struct YTCreator {
        pub names: Vec<String>,
        pub avatar_links: Vec<String>,
        pub descriptions: Vec<String>,
        pub is_live_status: Vec<String>,
        pub subscribers: Vec<String>
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AllowedFieldNamesForSorting {
    Subscribers,
    IsLiveStatus,
}

impl Default for AllowedFieldNamesForSorting {
    fn default() -> Self {
        AllowedFieldNamesForSorting::Subscribers
    }
}

impl YTCreator {
    fn size(&self) -> usize {
        let mut lengths_all: Vec<usize> = vec![];
        let mut msges: String = "".to_string();
        for field_name in YTCreator::field_names().iter() {
            let len_field: usize = self.get_field(field_name).unwrap().len();
            lengths_all.push(len_field);
            if len_field > MAX_EXPECTED_ITEMS {
                msges += format!(
                    "Found: {len_field} but got {MAX_EXPECTED_ITEMS} for the given field_name: {field_name}\n"
                ).as_str();
            };
        }

        if !msges.is_empty() {
            panic!("Found more items than expected. {msges}");
        }
        assert!(
            lengths_all
                .windows(2)
                .all(|single_len| single_len[0] == single_len[1]),
            "Not all fields have equal length. Check the input data again."
        );
        self.names.len()
    }

    // Implement sort_by(field_name="")
    // For example, sort_by(field_name="subscriber_count")
    // Inplace operation

    fn helper_sort(&self, field: &[String], is_bool: bool) -> (YTCreator, Vec<usize>) {
        let mut new_yt_creator: YTCreator = YTCreator::default();
        // let's create a mapping =>
        // {0 : field[...], 1: field[...], ...}
        let mut hash_map: HashMap<i32, String> = HashMap::new();
        for (idx, val) in field.iter().enumerate() {
            hash_map.insert(idx as i32, val.to_string());
        }
        // println!("Hash Map at this stage: {:?}", hash_map);
        let mut count_vec: Vec<(&i32, &String)> = hash_map.iter().collect();
        count_vec.sort_by(|a, b| {
            if b.1 == a.1 {
                let b_32: i32 = self
                    .subscribers
                    .get(*b.0 as usize)
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                let a_32: i32 = self
                    .subscribers
                    .get(*a.0 as usize)
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();
                b_32.cmp(&a_32)
            } else {
                match is_bool {
                    true => {
                        let b_bool: bool = b.1.parse().ok().unwrap();
                        let a_bool: bool = a.1.parse().ok().unwrap();
                        b_bool.cmp(&a_bool)
                    }
                    false => {
                        let b_32: i32 = b.1.parse().ok().unwrap();
                        let a_32: i32 = a.1.parse().ok().unwrap();
                        b_32.cmp(&a_32)
                    }
                }
            }
        });
        // println!("After sorting: {:?}", count_vec);
        for field_name in YTCreator::field_names() {
            let data_field = self.get_field(field_name).unwrap();
            let mut new_data_field = Vec::<String>::new();
            for tuple_ in count_vec.iter() {
                let idx = tuple_.0;
                new_data_field.push(data_field.get(*idx as usize).unwrap().to_string());
            }
            new_yt_creator.set_field(field_name, new_data_field);
        }
        let mut new_indices = Vec::<usize>::new();
        for tuple_ in count_vec.iter() {
            let idx = tuple_.0;
            new_indices.push(*idx as usize);
        }
        println!("new indices: {:?}", new_indices);
        println!("before it looked like: {:?}", self.is_live_status);
        (new_yt_creator, new_indices)
    }

    pub fn sort_by(
        &self,
        field_name: AllowedFieldNamesForSorting,
    ) -> Option<(YTCreator, Vec<usize>)> {
        match field_name {
            AllowedFieldNamesForSorting::Subscribers => {
                let field = self.get_field("subscribers").unwrap();
                let sort_output = self.helper_sort(field, false);
                Some(sort_output)
            }
            AllowedFieldNamesForSorting::IsLiveStatus => {
                let field = self.get_field("is_live_status").unwrap();
                let sort_output = self.helper_sort(field, true);
                Some(sort_output)
            }
        }
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
    SortOptionChanged(AllowedFieldNamesForSorting),
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

struct ContainerCustomStyle {
    curr_theme: theme::Theme,
    curr_live_status: bool,
}

const DARK_BACKGROUND_LIVE: Option<iced_core::Background> =
    Some(iced_core::Background::Color(Color {
        r: 255.0,
        g: 255.0,
        b: 255.0,
        a: 0.7,
    }));
const LIGHT_BACKGROUND_LIVE: Option<iced_core::Background> =
    Some(iced_core::Background::Color(Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.9,
    }));

impl container::StyleSheet for ContainerCustomStyle {
    type Style = theme::Theme;
    fn appearance(&self, _: &iced::Theme) -> container::Appearance {
        let (text_color, bg) = match &self.curr_live_status {
            true => match &self.curr_theme {
                iced::Theme::Light => (Color::WHITE, LIGHT_BACKGROUND_LIVE),
                iced::Theme::Dark => (Color::BLACK, DARK_BACKGROUND_LIVE),
                iced::Theme::Custom(_) => (
                    Color::BLACK,
                    Some(iced_core::Background::Color(Color::TRANSPARENT)),
                ),
            },
            false => match &self.curr_theme {
                iced::Theme::Light => (Color::BLACK, None),
                iced::Theme::Dark => (Color::WHITE, None),
                iced::Theme::Custom(_) => (
                    Color::BLACK,
                    Some(iced_core::Background::Color(Color::TRANSPARENT)),
                ),
            },
        };
        container::Appearance {
            text_color: Some(text_color),
            background: bg,
            border_radius: 2.0,
            border_width: 2.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

pub fn create_row(
    cards: &ListOfCards,
    img_handles_row: &[image::Handle],
    offset: usize,
    theme: &theme::Theme,
    status: &[bool],
) -> Row<'static, Message> {
    Row::with_children(
        cards
            .cards
            .iter()
            .enumerate()
            .map(|(idx, each_card)| {
                container(
                    row![
                        column![create_card(each_card)]
                            .padding(20)
                            .width(Length::FillPortion(2)),
                        column![profile_pic(
                            130,
                            img_handles_row.get(offset + idx).unwrap().to_owned()
                        )]
                        .width(Length::FillPortion(1))
                        .padding(20)
                    ]
                    .align_items(iced::Alignment::Center)
                    .height(Length::Fill),
                )
                .width(Length::Fill)
                .center_y()
                .style(iced::theme::Container::Custom(Box::new(
                    ContainerCustomStyle {
                        curr_theme: theme.clone(),
                        curr_live_status: *status.get(idx + offset).unwrap(),
                    },
                )))
                .into()
            })
            .collect(),
    )
}

pub fn profile_pic<'a>(width: u16, img_handle: image::Handle) -> Container<'a, Message> {
    container(
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
        };
        let out_img: image::Handle = image::Handle::from_memory(img_bytes.expect("Probably the image wasn't read successfully. Please check the avatar link again").to_vec());
        out_handles.push(out_img);
    }

    out_handles
}

pub fn get_live_status(live_status: Option<&Vec<String>>) -> Vec<bool> {
    let mut out_status: Vec<bool> = Vec::new();
    let status_as_strings = live_status.expect("Didn't find any data in is_live_status field.");
    for status in status_as_strings {
        let _true_str = String::from("true");
        let _false_str = String::from("false");
        let bool_output: bool = status.trim().parse().unwrap();
        out_status.push(bool_output);
    }
    out_status
}

#[cfg(test)]
mod test {
    use super::*;

    // Testing YTCreator struct methods
    #[test]
    fn test_yt_creator_size_empty() {
        let mock_yt_creator = YTCreator::default();
        assert_eq!(mock_yt_creator.size(), 0);
    }

    #[test]
    fn test_yt_creator_size_non_empty() {
        let mock_yt_creator = YTCreator {
            names: vec!["Kush".to_string()],
            avatar_links: vec!["sample".to_string()],
            descriptions: vec!["Dev".to_string()],
            is_live_status: vec!["true".to_string()],
            subscribers: vec!["200".to_string()],
        };
        assert_eq!(mock_yt_creator.size(), 1);
    }

    #[test]
    #[should_panic(expected = "Not all fields have equal length")]
    fn test_yt_creator_size_invalid() {
        YTCreator {
            names: vec!["Kush".to_string(), "Another".to_string()],
            avatar_links: vec!["sample".to_string()],
            descriptions: vec!["Dev".to_string(), "Another".to_string()],
            is_live_status: vec!["true".to_string()],
            subscribers: vec!["200".to_string()],
        }
        .size();
    }

    #[test]
    fn test_yt_creator_get_field_names() {
        assert_eq!(
            YTCreator::field_names(),
            vec![
                "names",
                "avatar_links",
                "descriptions",
                "is_live_status",
                "subscribers"
            ]
        );
    }

    #[test]
    fn test_yt_creator_get_field() {
        let mock_yt_creator = get_json_data(Some("test_assets/sample_data.json"));
        let output_names = mock_yt_creator.get_field("names");
        assert!(output_names.is_some());
        // Testing for just one field is enough IMO
        assert_eq!(output_names.unwrap(), &vec!["Kush", "Kushashwa"]);
    }

    #[test]
    fn test_yt_creator_get_field_invalid() {
        let mock_yt_creator = get_json_data(Some("test_assets/sample_data.json"));
        assert!(mock_yt_creator.get_field("doesn't_exist").is_none());
    }

    #[test]
    fn test_get_json_data_valid_file() {
        let expected_output: YTCreator = YTCreator {
            names: vec!["Kush", "Kushashwa"].iter().map(|&s|s.into()).collect(),
            avatar_links: vec!["https://avatars.githubusercontent.com/u/19997320?v=4", "https://media-exp1.licdn.com/dms/image/C4D03AQGiAbH1TT3fNA/profile-displayphoto-shrink_800_800/0/1642226109876?e=2147483647&v=beta&t=fcJojobq-NZv0oNX_WW9RrCsYsoTqz0TSYMcC6zOGco"].iter().map(|&s|s.into()).collect(),
            descriptions: vec!["Developer", "Developer"].iter().map(|&s|s.into()).collect(),
            is_live_status: vec!["true", "false"].iter().map(|&s|s.into()).collect(),
            subscribers: vec!["100", "200"].iter().map(|&s|s.into()).collect()
        };
        assert_eq!(
            get_json_data(Some("test_assets/sample_data.json")),
            expected_output
        );
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn test_get_json_data_invalid_file() {
        get_json_data(Some("invalid_files.json"));
    }

    #[test]
    fn test_get_all_avatars_valid() {
        let sample_data_yt_creator: YTCreator = get_json_data(Some("test_assets/sample_data.json"));
        assert!(!get_all_avatars(&sample_data_yt_creator).is_empty());
    }

    #[test]
    fn test_get_all_avatars_empty_data() {
        let sample_data_yt_creator: YTCreator = get_json_data(Some("test_assets/empty_data.json"));
        assert!(get_all_avatars(&sample_data_yt_creator).is_empty());
    }

    #[test]
    #[should_panic(expected = "wasn't read successfully")]
    fn test_get_all_avatars_invalid_data() {
        let mut sample_data_yt_creator: YTCreator =
            get_json_data(Some("test_assets/sample_data.json"));
        sample_data_yt_creator.avatar_links.pop();
        sample_data_yt_creator
            .avatar_links
            .push("wrong_link".to_string());
        get_all_avatars(&sample_data_yt_creator);
    }

    #[test]
    fn test_get_live_status_valid() {
        let sample_data_yt_creator: YTCreator = get_json_data(Some("test_assets/sample_data.json"));
        assert_eq!(
            get_live_status(Some(&sample_data_yt_creator.is_live_status)),
            vec![true, false]
        );
    }

    #[test]
    fn test_get_live_status_empty() {
        let sample_data_yt_creator: YTCreator = get_json_data(Some("test_assets/empty_data.json"));
        assert_eq!(
            get_live_status(Some(&sample_data_yt_creator.is_live_status)).len(),
            0
        );
    }

    #[test]
    #[should_panic(expected = "Didn't find any data in is_live_status field.")]
    fn test_get_live_status_invalid() {
        get_live_status(None);
    }

    #[test]
    fn test_yt_creator_slice_to() {
        let yt_creator_mock: YTCreator = get_json_data(Some("test_assets/sample_data.json"));
        assert_eq!(yt_creator_mock.slice_to(1).size(), 1);
    }

    #[test]
    #[should_panic(expected = "Not enough elements to be sliced into")]
    fn test_yt_creator_slice_to_more_than_existing() {
        let yt_creator_mock: YTCreator = get_json_data(Some("test_assets/sample_data.json"));
        yt_creator_mock.slice_to(3);
    }

    #[test]
    fn test_yt_creator_slice_to_on_empty_valid() {
        let yt_creator_mock: YTCreator = get_json_data(Some("test_assets/empty_data.json"));
        assert_eq!(yt_creator_mock.slice_to(0).size(), 0);
    }

    #[test]
    #[should_panic(expected = "Not enough elements to be sliced into")]
    fn test_yt_creator_slice_to_on_empty_invalid() {
        let yt_creator_mock: YTCreator = get_json_data(Some("test_assets/empty_data.json"));
        assert_eq!(yt_creator_mock.slice_to(2).size(), 0);
    }

    #[test]
    fn test_yt_creator_set_field() {
        let mut yt_creator_mock: YTCreator = get_json_data(Some("test_assets/more_data.json"));
        let new_data: Vec<String> = vec![
            "0".to_string(),
            "0".to_string(),
            "0".to_string(),
            "0".to_string(),
        ];
        yt_creator_mock.set_field("subscribers", new_data);
        assert_eq!(yt_creator_mock.subscribers, vec!["0", "0", "0", "0"]);
    }

    #[test]
    fn test_yt_creator_sort_by_is_live_status() {
        let yt_creator_mock: YTCreator = get_json_data(Some("test_assets/more_data.json"));
        let sorted_tuple_with_indices =
            yt_creator_mock.sort_by(AllowedFieldNamesForSorting::IsLiveStatus);
        let new_yt_creator_mock = sorted_tuple_with_indices.unwrap().0;
        assert_eq!(
            new_yt_creator_mock.subscribers,
            ["300", "100", "400", "200"]
        );
    }

    #[test]
    fn test_yt_creator_sort_by_subscribers() {
        let yt_creator_mock: YTCreator = get_json_data(Some("test_assets/more_data.json"));
        let new_yt_creator_mock = yt_creator_mock
            .sort_by(AllowedFieldNamesForSorting::Subscribers)
            .unwrap()
            .0;
        assert_ne!(
            new_yt_creator_mock.avatar_links,
            yt_creator_mock.avatar_links
        );
        assert_eq!(
            new_yt_creator_mock.subscribers,
            ["400", "300", "200", "100"]
        );
    }
}
