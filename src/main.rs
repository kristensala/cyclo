use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use bluetoothctl::{BluetoothError, Btle, listen_events, Device};

pub mod bluetoothctl;
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

#[derive(Clone, Debug)]
pub struct App {
    btle: Option<Btle>,
    state: Arc<Mutex<State>>,
    tick: Tick,
    display_heart_rate: u8,
    display_power: u8,
    display_scanned_devices: Vec<Device>,
    connected_devices: Vec<Device>
}

#[derive(Debug, Clone)]
enum Tick {
    Idle,
    Listen
}

#[derive(Debug, Clone)]
pub enum Message {
    InitBluetooth(Result<Btle, BluetoothError>),
    ScanDevices,
    FoundDevices(Result<Vec<Device>, ()>),
    Connect,
    Disconnect,
    ListenEvents,
    ReadData(Result<(), BluetoothError>),
    Tick(Instant)
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
                state: Arc::new(Mutex::new(State::new())),
                tick: Tick::Listen,
                display_heart_rate: 0,
                display_power: 0,
                display_scanned_devices: Vec::new(),
                connected_devices: Vec::new()
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
                return Command::perform(self.btle.clone().unwrap().scan(), Message::FoundDevices)
            }
            Message::FoundDevices(resp) => {
                println!("scanned {:?}", resp);
                if let Ok(value) = resp {
                    let connected_devices = value.clone()
                        .into_iter()
                        .filter(|x| x.is_connected)
                        .collect::<Vec<Device>>();

                    self.display_scanned_devices = value;
                    self.connected_devices = connected_devices;
                }
            }
            Message::ListenEvents => {
                let btle = self.btle.clone().unwrap();
                let state = Arc::clone(&self.state);

                return Command::perform(listen_events(btle.adapter, state), Message::ReadData);
            }
            Message::ReadData(_) => {
                println!("Started listening data");
            }
            Message::Tick(_) => {
                let clone = Arc::clone(&self.state);
                let lock = clone.lock().unwrap();
                self.display_heart_rate = lock.heart_rate;
                //todo set display power
            }
            _ => {

            }
        }

        return Command::none();
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.tick {
            Tick::Idle=> Subscription::none(),
            Tick::Listen => {
                time::every(Duration::from_millis(500)).map(Message::Tick)
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let scan_btn = button("Scan")
            .on_press(Message::ScanDevices)
            .padding(5.);

        let display_devices = self.display_scanned_devices.clone();
        let scanned_devices = column(
            display_devices
                .into_iter()
                .map(|device| {
                    row![text(format!("{} {} {}", device.name, device.address, device.is_connected))]
                        .spacing(10)
                        .into()
                }).collect()
        );

        let listen_btn = button("Listen")
            .on_press(Message::ListenEvents)
            .padding(5.);

        let heart_beat = text(self.display_heart_rate).size(40);

        let content = column![
            scan_btn,
            scanned_devices,
            listen_btn,
            heart_beat
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


