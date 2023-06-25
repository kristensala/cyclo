use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use bluetoothctl::{BluetoothError, Btle, listen_events};

pub mod bluetoothctl;
pub mod state;
pub mod device;

use device::Device;
use iced::theme::{self, Theme};
use iced::{executor, subscription, time};
use iced::widget::{
    button, checkbox, column, container, pick_list, row, slider, text, vertical_space, scrollable,
};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
};
use state::State;

#[derive(Debug, Clone)]
struct Stopwatch {
    duration: Duration,
    state: StopwatchState
}

#[derive(Debug, Clone)]
enum StopwatchState {
    Idle,
    Ticking { last_tick: Instant }
}

#[derive(Clone, Debug)]
struct App {
    btle: Option<Btle>,
    state: Arc<Mutex<State>>,
    tick: Tick,
    display_heart_rate: u8,
    display_power: u8, // don't know the datatype currently
    display_scanned_devices: Vec<Device>,
    connected_devices: Vec<Device>,
    errors: Vec<String>,
    stopwatch: Stopwatch
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

impl Stopwatch {
    fn new() -> Stopwatch {
        return Stopwatch {
            duration: Duration::default(),
            state: StopwatchState::Idle
        }
    }
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
                connected_devices: Vec::new(),
                errors: Vec::new(),
                stopwatch: Stopwatch::new()
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
                match resp {
                    Ok(value) => {
                        self.btle = Some(value);
                    },
                    Err(err) => {
                        //todo: make a ui section to display errors
                        self.errors.push(err.to_string())
                    }
                }
            }
            Message::ScanDevices => {
                match self.btle.clone() {
                    Some(value) => {
                        return Command::perform(value.scan(), Message::FoundDevices)
                    },
                    None => {
                        self.errors.push(String::from("Btle has none value"));
                    }
                    
                }
            }
            Message::FoundDevices(resp) => {
                println!("scanned {:?}", resp);
                if let Ok(value) = resp {
                    let connected_devices = value.clone()
                        .into_iter()
                        .filter(|x| x.is_connected)
                        .collect::<Vec<Device>>();

                    self.display_scanned_devices = value;

                    // add only valid devices to the list 
                    // like hr monitor and turbo trainer
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
            Message::Tick(now) => {
                let clone = Arc::clone(&self.state);
                let lock = clone.lock().unwrap();
                self.display_heart_rate = lock.heart_rate;
                //todo set display power

                if let StopwatchState::Ticking { last_tick } = &mut self.stopwatch.state {
                    self.stopwatch.duration += now - *last_tick;
                    *last_tick = now;
                }
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
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;

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

        //todo: start listening automatically when device is connected
        let listen_btn = button("Listen")
            .on_press(Message::ListenEvents)
            .padding(5.);

        let heart_beat = text(self.display_heart_rate).size(40);

        let seconds = self.stopwatch.duration.as_secs();
        let stopwatch = text(format!(
            "{:0>2}:{:0>2}:{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE
        ))
        .size(40);

        let content = column![
            scan_btn,
            scanned_devices,
            listen_btn,
            heart_beat,
            stopwatch
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

    Device::get_class(String::from("00240404"));
    return App::run(Settings::default());
}


