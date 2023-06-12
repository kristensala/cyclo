use std::io::stdout;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use bluetoothctl::{BluetoothError};

pub mod bluetoothctl;
pub mod device;

use btleplug::platform::Adapter;
use iced::theme::{self, Theme};
use iced::executor;
use iced::widget::{
    button, checkbox, column, container, pick_list, row, slider, text, vertical_space, scrollable,
};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
};


pub struct App {
    bluetooth_adapter: Arc<Mutex<Option<Adapter>>>
}

#[derive(Debug, Clone)]
pub enum Message {
    InitBluetooth(Result<Adapter, BluetoothError>),
    DiscoverDevices,
    ScanDevices,
    DevicesScanned(Result<(), BluetoothError>),
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
                bluetooth_adapter: Arc::new(Mutex::new(None))
            },
            Command::perform(bluetoothctl::init(), Message::InitBluetooth)
        )
    }

    fn title(&self) -> String {
        return String::from("Cyclo");
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InitBluetooth(resp) => {
                println!("adapter {:?}", resp);
                *self.bluetooth_adapter.lock().unwrap() = Some(resp.unwrap());
            }
            Message::ScanDevices => {
                let adapter_guard = self.bluetooth_adapter.lock().unwrap();
                let clone = adapter_guard.clone();

                //println!("adapter {:?}", clone);
                return Command::perform(bluetoothctl::scan(clone), Message::DevicesScanned)
            }
            Message::DevicesScanned(resp) => {

            }
            _ => {
            }
        }

        return Command::none();
    }

    /*fn subscription(&self) -> Subscription<Self::Message> {
        
    }*/

    
    fn view(&self) -> Element<Message> {
        let scan_btn = button("Scan").on_press(Message::ScanDevices).padding(5.);
        let content = column![
            scan_btn,
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10);


        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    return App::run(Settings::default());
}


