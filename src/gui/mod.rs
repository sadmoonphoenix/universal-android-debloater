pub mod style;
pub mod views;
pub mod widgets;

pub use views::about::About as AboutView;
pub use views::list::{List as AppsView, Message as AppsMessage};
pub use views::settings::{Settings as SettingsView, Message as SettingsMessage};
pub use crate::core::uad_lists::{ load_debloat_lists, Package };
pub use crate::core::sync::{ get_phone_brand};
use std::{collections::HashMap};
use static_init::{dynamic};

use iced::{
    button, Alignment, Application, Button, Column, Command, Space,
    Container, Element, Length, Row, Settings, Text, window::Settings as Window, Svg,
};

#[dynamic]
static UAD_LISTS: HashMap<String, Package> = load_debloat_lists();

#[derive(Debug, Clone)]
pub enum View {
    List,
    About,
    Settings,   
}

impl Default for View {
    fn default() -> Self {
        Self::List
    }
}

#[derive(Debug, Clone)]
pub struct UadGui {
    view: View,
    apps_view: AppsView,
    about_view: AboutView,
    settings_view: SettingsView,
    input_value: String,
    about_btn: button::State,
    settings_btn: button::State,
    apps_btn: button::State,
    apps_refresh_btn: button::State,
    device_name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation Panel
    AboutPressed,
    SettingsPressed,
    AppsRefreshPress,
    AppsPress,

    AppsAction(AppsMessage),
    SettingsAction(SettingsMessage),
    Init(AppsMessage),
}

impl Default for UadGui {
    fn default() -> Self {
        Self {
            view: View::default(),
            apps_view: AppsView::default(),
            about_view: AboutView::default(),
            settings_view: SettingsView::default(),
            input_value: "".to_string(),
            device_name: "No phone connected".to_string(),
            about_btn: button::State::default(),
            settings_btn: button::State::default(),
            apps_btn: button::State::default(),
            apps_refresh_btn: button::State::default(),
        }
    }
}

impl Application for UadGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::perform(Self::load_phone_packages(), Message::Init),
        )
    }

    fn title(&self) -> String {
        String::from("UadGui")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Init(_) => {
                self.device_name = get_phone_brand();
                info!("PHONE_MODEL: {}", self.device_name);
                Command::perform(Self::load_phone_packages(), Message::AppsAction)
            }
            Message::AppsRefreshPress => {
                self.device_name = get_phone_brand();
                info!("PHONE_MODEL: {}", self.device_name);
                self.apps_view = AppsView::default();
                self.view = View::List;
                Command::perform(Self::load_phone_packages(), Message::AppsAction)
            }
            Message::AppsPress => {
                self.view = View::List;
                self.apps_view.update(AppsMessage::LoadSettings(self.settings_view)).map(Message::AppsAction)
            }
            Message::AboutPressed => {
                self.view = View::About;
                Command::none()
            }
            Message::SettingsPressed => {
                self.view = View::Settings;
                Command::none()
            }
            Message::AppsAction(msg) => {
                self.apps_view.update(msg).map(Message::AppsAction)
            }
            Message::SettingsAction(msg) => {
                self.settings_view.update(msg);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let add_svg_path = format!("{}/ressources/assets/refresh.svg", env!("CARGO_MANIFEST_DIR"));
        let refresh_list_icon = Svg::from_path(add_svg_path);

        let apps_btn = Button::new(&mut self.apps_btn, Text::new("Apps"))
            .on_press(Message::AppsPress)
            .padding(5)
            .style(style::PrimaryButton::Enabled);

        let apps_refresh_btn = Button::new(&mut self.apps_refresh_btn, refresh_list_icon)
            .on_press(Message::AppsRefreshPress)
            .padding(5)
            .style(style::PrimaryButton::Enabled);

        let about_btn = Button::new(&mut self.about_btn, Text::new("About"))
            .on_press(Message::AboutPressed)
            .padding(5)
            .style(style::PrimaryButton::Enabled);

        let settings_btn = Button::new(&mut self.settings_btn, Text::new("Settings"))
            .on_press(Message::SettingsPressed)
            .padding(5)
            .style(style::PrimaryButton::Enabled);

        let row = Row::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10)
            .push(Text::new("Device: ".to_string() + &self.device_name))
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(apps_refresh_btn)
            .push(apps_btn)
            .push(about_btn)
            .push(settings_btn);

        let navigation_container = Container::new(row)
            .width(Length::Fill)
            .padding(10)
            .style(style::NavigationContainer);

        let main_container = match self.view {
            View::List => self.apps_view.view().map(Message::AppsAction),
            View::About => self.about_view.view(),
            View::Settings => self.settings_view.view().map(Message::SettingsAction),
        };

        Column::new()
            .width(Length::Fill)
            .push(navigation_container)
            .push(main_container)
            .into()
    }
}


impl UadGui {
    pub fn start() {
        let settings: Settings<()> = Settings {
            window: Window {
                size: (1050, 800),
                resizable: true,
                decorations: true,
                ..iced::window::Settings::default()
            },
            default_text_size: 17,
            ..iced::Settings::default()
        };
        Self::run(settings).unwrap_err();
    }

    pub async fn load_phone_packages() -> AppsMessage {
        AppsMessage::LoadPackages(&UAD_LISTS)
    }
}