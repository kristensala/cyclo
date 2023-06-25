use std::sync::{Arc, Mutex};
use std::time::Duration;

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter, CharPropFlags, CentralEvent};
use btleplug::platform::{Manager, Adapter};
use btleplug::api::bleuuid::uuid_from_u16;
use futures::stream::StreamExt;
use thiserror::Error;
use tokio::time;

use crate::device::{Device, MinorDeviceClass};
use crate::state::State;

const HEART_RATE_SERVICE: uuid::Uuid = uuid_from_u16(0x180D);
const HEART_RATE_CHARACTERISTICS: uuid::Uuid = uuid_from_u16(0x2A37);

#[derive(Error, Debug, Clone)]
pub enum BluetoothError {
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
    #[error("Adapter not found")]
    AdapterNotFound
}


#[derive(Debug, Clone)]
pub struct Btle {
    pub adapter: Adapter,
}

impl Btle {
    pub async fn init() -> Result<Btle, BluetoothError> {
        let manager = Manager::new().await.expect("Failed to initialize manager!");
        let adapters_list = manager.adapters().await.unwrap();

        let mut adapter: Option<Adapter> = None;
        if adapters_list.len() == 1 {
            adapter = match adapters_list.first() {
                Some(x) => Some(x.to_owned()),
                None => None
            };
        }

        if let Some(value) = adapter {
            //listen_events(value.clone(), state).await;

            return Ok(Btle {
                adapter: value,
            });
        }

        return Err(BluetoothError::AdapterNotFound);
    }

    pub async fn scan(self) -> Result<Vec<Device>, ()> {
        self.adapter.start_scan(ScanFilter::default()).await
            .expect("Can't scan BLE adapter for connected devices...");

        time::sleep(Duration::from_secs(10)).await;
        _ = self.adapter.stop_scan().await;

        let devices = self.adapter.peripherals().await.unwrap();

        let mut result: Vec<Device> = Vec::new();
        for device in devices {
            let props_result = device.properties().await;
            if let Ok(props_option) = props_result {
                if let Some(props) = props_option {
                    let custom_device = Device {
                        name: props.local_name.unwrap_or("unknown".to_string()),
                        address: props.address.to_string(),
                        is_connected: device.is_connected().await.unwrap_or(false),
                        minor_device_class: MinorDeviceClass::Unknown
                    };
                    result.push(custom_device);
                }
            }
        }

        return Ok(result);
    }
}

pub async fn listen_events(adapter: Adapter, state: Arc<Mutex<State>>) -> Result<(), BluetoothError> {
    let mut events = match adapter.events().await {
        Ok(value) => value,
        Err(error) => return Err(BluetoothError::UnexpectedError(error.to_string()))
    };

    tokio::spawn(async move {
        println!("Starting a new listening thread");

        while let Some(event) = events.next().await {
            match event {
                CentralEvent::DeviceConnected(id) => {
                    println!("connected to device {:?}", id);

                    let peripehral = adapter.peripheral(&id).await.unwrap();
                    peripehral.discover_services().await.unwrap();

                    let heart_rate_service = peripehral.services()
                        .into_iter()
                        .find(|service| service.uuid == HEART_RATE_SERVICE);

                    if heart_rate_service.is_some() {
                        let ch = heart_rate_service.unwrap().characteristics
                            .into_iter()
                            .find(|charateristic| charateristic.properties.contains(CharPropFlags::NOTIFY) 
                                && charateristic.uuid == HEART_RATE_CHARACTERISTICS);

                        if ch.is_none() {
                            println!("Heart Rate Characteristic not found!");
                            break;
                        }

                        let hr_characteristic = &ch.unwrap();

                        peripehral.subscribe(hr_characteristic).await.unwrap();

                        let heart_rate_ch = hr_characteristic.clone();
                        let mut notification_stream = peripehral.notifications().await.unwrap();

                        let state_clone = Arc::clone(&state);
                        tokio::spawn(async move {
                            println!("Starting a new getting data thread");

                            while let Some(notification) = notification_stream.next().await {
                                if notification.uuid == heart_rate_ch.uuid {
                                    let data = notification.value;

                                    if data.len() < 3 { //TODO: not sure if i need this check
                                        println!("Invalid data - [{}] {:?}", notification.uuid, data);
                                        continue;
                                    }

                                    //TODO: read => https://stackoverflow.com/questions/65443033/heart-rate-value-in-ble/65458794?noredirect=1#comment118474300_65458794
                                    let res = data[1]; // heart rate

                                    let mut state_lock = state_clone.lock().unwrap();
                                    state_lock.heart_rate = res;
                                    state_lock.heart_rate_history.push(res);
                                }
                            }
                        });
                    }
                }
                _ => {
                }
            }
        }
    });

    return Ok(());
}

