use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use bluetoothctl::{BluetoothError, Btle, listen_events};
use tokio_stream::{self as stream, StreamExt};

pub mod bluetoothctl;
pub mod device;
pub mod state;

use iced::theme::{self, Theme};
use iced::{executor, subscription, time};
use iced::widget::{
    button, checkbox, column, container, pick_list, row, slider, text, vertical_space, scrollable,
};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
};
use state::State;


struct AppState {
    current_heart_rate: Option<u8>,
    current_power: Option<u8>
}

#[derive(Clone, Debug)]
pub struct App {
    btle: Option<Btle>,
    scanned_devices: Vec<String>,
    state: Arc<Mutex<State>>,
    listen: Listen,
    heart_beat: u8
}

#[derive(Debug, Clone)]
enum Listen {
    Idle,
    Heartbeat
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
    Heartbeat(Instant)
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
                state: Arc::new(Mutex::new(State::new())),
                listen: Listen::Heartbeat,
                heart_beat: 0
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
                println!("Started listending events");
                
            }
            Message::Heartbeat(_) => {
                // display heartrate on ui
                let clone = Arc::clone(&self.state);
                let lock = clone.lock().unwrap();
                self.heart_beat = lock.heart_rate;
            }
            _ => {

            }
        }

        return Command::none();
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.listen {
            Listen::Idle=> Subscription::none(),
            Listen::Heartbeat => {
                // trigger hearbeat message on every second
                time::every(Duration::from_secs(1)).map(Message::Heartbeat)
            },
        }
    }

    
    fn view(&self) -> Element<Message> {
        let scan_btn = button("Scan").on_press(Message::ScanDevices).padding(5.);
        let listen_btn = button("Listen").on_press(Message::ListenEvents).padding(5.);

        let beat = text(self.heart_beat).size(40);

        let content = column![
            scan_btn,
            listen_btn,
            beat
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


