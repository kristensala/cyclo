use std::io::stdout;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use bluetoothctl::{BluetoothError, Btle};

pub mod bluetoothctl;
pub mod device;

use btleplug::api::Peripheral;
use btleplug::platform::Adapter;
use device::Device;
use iced::theme::{self, Theme};
use iced::executor;
use iced::widget::{
    button, checkbox, column, container, pick_list, row, slider, text, vertical_space, scrollable,
};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
};

#[derive(Clone, Debug)]
pub struct App {
    btle: Arc<Mutex<Option<Btle>>>,
    scanned_devices: Vec<String>

}

#[derive(Debug, Clone)]
pub enum Message {
    InitBluetooth(Result<Btle, BluetoothError>),
    DiscoverDevices,
    ScanDevices,
    DevicesScanned(Result<Vec<String>, ()>),
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
                btle: Arc::new(Mutex::new(None)),
                scanned_devices: Vec::new()
            },
            Command::perform(Btle::init(), Message::InitBluetooth)
        )
    }

    fn title(&self) -> String {
        return String::from("Cyclo");
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InitBluetooth(resp) => {
                println!("adapter {:?}", resp);
                //*self.bluetooth_adapter.lock().unwrap() = Some(resp.unwrap());
                *self.btle.lock().unwrap() = Some(resp.unwrap());

            }
            Message::ScanDevices => {
                let btle_guard = self.btle.lock().unwrap();
                let clone = btle_guard.clone().unwrap();

                return Command::perform(clone.scan(), Message::DevicesScanned)
            }
            Message::DevicesScanned(resp) => {
                self.scanned_devices = resp.unwrap();
                println!("scanned {:?}", self);
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


