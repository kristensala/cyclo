use std::io::stdout;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use bluetoothctl::{BluetoothError, Btle, listen_events};

pub mod bluetoothctl;
pub mod device;
pub mod state;

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
use state::State;

#[derive(Clone, Debug)]
pub struct App {
    btle: Option<Btle>,
    scanned_devices: Vec<String>,
    state: Arc<Mutex<State>>

}

#[derive(Debug, Clone)]
pub enum Message {
    InitBluetooth(Result<Btle, BluetoothError>),
    DiscoverDevices,
    ScanDevices,
    DevicesScanned(Result<Vec<String>, ()>),
    Connect,
    Disconnect,
    ListenEvents,
    ReadData(Result<(), BluetoothError>),
    GetState
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                btle: None,
                scanned_devices: Vec::new(),
                state: Arc::new(Mutex::new(State::new()))
            },
            Command::perform(Btle::init(), Message::InitBluetooth)
        )
    }

    fn title(&self) -> String {
        return String::from("Cyclo");
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        println!("update");
        match message {
            Message::InitBluetooth(resp) => {
                println!("adapter {:?}", resp);
                self.btle = Some(resp.unwrap());

            }
            Message::ScanDevices => {
                return Command::perform(self.btle.clone().unwrap().scan(), Message::DevicesScanned)
            }
            Message::DevicesScanned(resp) => {
                self.scanned_devices = resp.unwrap();
                println!("scanned {:?}", self);
            }
            Message::ListenEvents => {
                let btle = self.btle.clone().unwrap();
                let state = Arc::clone(&self.state);

                return Command::perform(listen_events(btle.adapter, state), Message::ReadData);
            }
            Message::ReadData(resp) => {
                
            }
            Message::GetState => {
                let state = Arc::clone(&self.state);
                let lock = state.lock().unwrap();
                println!("catch: {:?}", lock.heart_rate);
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
        let listen_btn = button("Listen").on_press(Message::ListenEvents).padding(5.);
        let get_state_btn = button("Listen").on_press(Message::GetState).padding(5.);

        let content = column![
            scan_btn,
            listen_btn,
            get_state_btn
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


