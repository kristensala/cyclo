use iced::theme::{self, Theme};
use iced::executor;
use iced::time;
use iced::widget::{
    button, checkbox, column, container, pick_list, row, slider, text, vertical_space, scrollable,
};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
};


pub struct App {
    connected_devices: Vec<String>
}

#[derive(Debug)]
pub enum Message {
    DiscoverDevices,
    ScanDevices,
    Connect,
    Disconnect
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                connected_devices: Vec::new()
            },
            Command::perform()
        )
    }

    fn title(&self) -> String {
        return String::from("Cyclo");
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Connect => {

            }
            _ => {
            }
        }

        return Command::none();
    }

    /*fn subscription(&self) -> Subscription<Self::Message> {
        
    }*/

    
    fn view(&self) -> Element<Message> {
        let content = column![
            vertical_space(600),
            "Scanned devices?",
            "",
            vertical_space(600),
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn start_ui() -> iced::Result {
    return App::run(Settings::default());
}

