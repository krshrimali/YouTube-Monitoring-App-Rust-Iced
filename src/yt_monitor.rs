use iced::theme::{self, Theme};
use iced::widget::{column, container, horizontal_rule, image, radio, row, text};
use iced::{Color, Length, Renderer, Sandbox};
#[path = "render_cards.rs"]
mod render_cards;

#[derive(Default)]
pub struct YTMonitor {
    theme: Theme,
    json_obj: render_cards::YTCreator,
    loaded_photos: Vec<image::Handle>,
    live_status: Vec<bool>,
}

// TODO: Make two separable users for female and males
// Not possible for YouTube but worth having for the library (later)
const DEFAULT_IMAGE_URL: &str = "https://www.w3schools.com/howto/img_avatar.png";

// TODO: Unused for now but can come handy later
fn default_img_handle(total_count: usize) -> Vec<image::Handle> {
    // TODO: Later on, accept gender as well
    let mut default_handles: Vec<image::Handle> = Vec::new();
    for _ in 0..total_count {
        let img_handle_link = DEFAULT_IMAGE_URL;
        let img_obj = reqwest::blocking::get(img_handle_link).ok();
        let img_bytes = match img_obj {
            Some(bytes) => bytes.bytes().ok(),
            None => None,
        }
        .unwrap();
        let out_img: image::Handle = image::Handle::from_memory(img_bytes.to_vec());
        default_handles.push(out_img);
    }
    default_handles
}

impl Sandbox for YTMonitor {
    type Message = render_cards::Message;

    fn new() -> YTMonitor {
        let json_obj = render_cards::get_json_data(None);
        let image_handles = render_cards::get_all_avatars(&json_obj);
        let statuses = render_cards::get_live_status(json_obj.get_field("is_live_status"));
        // Because dark as default is cool :D
        YTMonitor {
            theme: Theme::Dark,
            json_obj,
            loaded_photos: image_handles,
            live_status: statuses,
        }
    }

    fn title(&self) -> String {
        String::from("YT Monitoring App (by KRS)")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            render_cards::Message::ThemeChanged(theme) => {
                self.theme = match theme {
                    render_cards::ThemeType::Dark => Theme::Dark,
                    render_cards::ThemeType::Light => Theme::Light,
                    render_cards::ThemeType::Custom => Theme::custom(theme::Palette {
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
        let choose_theme = [
            render_cards::ThemeType::Dark,
            render_cards::ThemeType::Light,
            render_cards::ThemeType::Custom,
        ]
        .iter()
        .fold(
            row![text("Choose a theme:")].spacing(10),
            |column: iced_native::widget::row::Row<'_, render_cards::Message, Renderer>, theme| {
                column.push(radio(
                    format!("{:?}", theme),
                    *theme,
                    Some(match self.theme {
                        Theme::Dark => render_cards::ThemeType::Dark,
                        Theme::Light => render_cards::ThemeType::Light,
                        Theme::Custom { .. } => render_cards::ThemeType::Custom,
                    }),
                    render_cards::Message::ThemeChanged,
                ))
            },
        );

        let content = column![choose_theme]
            .spacing(20)
            .padding(20)
            .max_width(600)
            .width(Length::Fill);

        let footer = render_cards::create_text(
            "Thank you for being here, this was an app by Kushashwa Ravi Shrimali".to_string(),
            render_cards::TextType::Footer,
        );

        let title_header = render_cards::create_text(
            "Welcome! Here is the status of your favorite YouTubers:".to_string(),
            render_cards::TextType::Header,
        );

        let all_cards = render_cards::create_list_of_cards(&self.json_obj);
        let binding = render_cards::ListOfCards::default();
        let all_photos = self.loaded_photos.to_owned();
        let all_status = self.live_status.to_owned();

        let first_row = render_cards::create_row(
            all_cards.get(0).unwrap_or(&binding),
            &all_photos,
            0,
            &self.theme,
            &all_status,
        );
        let second_row = render_cards::create_row(
            all_cards.get(1).unwrap_or(&binding),
            &all_photos,
            4,
            &self.theme,
            &all_status,
        );
        let third_row = render_cards::create_row(
            all_cards.get(2).unwrap_or(&binding),
            &all_photos,
            8,
            &self.theme,
            &all_status,
        );

        container(
            column![
                content,
                horizontal_rule(10),
                title_header.height(Length::Shrink),
                horizontal_rule(10),
                column![first_row.height(Length::Fill), second_row.height(Length::Fill), third_row.height(Length::Fill)].height(Length::Fill),
                horizontal_rule(10),
                footer.height(Length::Shrink),
                horizontal_rule(10),
            ]
        )
        .height(Length::Shrink)
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
