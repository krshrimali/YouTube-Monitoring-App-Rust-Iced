use iced::theme::{self, Theme};
use iced::widget::{column, image, container, horizontal_rule, radio, row, text};
use iced::{Color, Length, Renderer, Sandbox};
#[path = "render_cards.rs"]
mod render_cards;

#[derive(Default)]
pub struct YTMonitor {
    theme: Theme,
    json_obj: render_cards::YTCreator,
    loaded_photos: Vec::<image::Handle>,
}

impl Sandbox for YTMonitor {
    type Message = render_cards::Message;

    fn new() -> YTMonitor {
        let json_obj = render_cards::get_json_data();
        let image_handles = render_cards::get_all_avatars(&json_obj);
        // Because dark as default is cool :D
        YTMonitor {
            theme: Theme::Dark,
            json_obj,
            loaded_photos: image_handles,
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

        let content = container(column![choose_theme].spacing(20).padding(20).max_width(600))
            .width(Length::Fill)
            .center_x();

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
        let first_row = render_cards::create_row(all_cards.get(0).unwrap_or(&binding), self.loaded_photos.clone());
        let second_row = render_cards::create_row(all_cards.get(1).unwrap_or(&binding), self.loaded_photos.clone());
        let third_row = render_cards::create_row(all_cards.get(2).unwrap_or(&binding), self.loaded_photos.clone());

        container(column![
            content,
            horizontal_rule(10),
            title_header,
            horizontal_rule(10),
            first_row,
            second_row,
            third_row,
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
