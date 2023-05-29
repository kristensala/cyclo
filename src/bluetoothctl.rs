use anyhow::{Result, anyhow, Ok};
use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter, CharPropFlags, CentralEvent, PeripheralProperties};
use btleplug::platform::{Manager, Adapter};
use btleplug::api::bleuuid::uuid_from_u16;
use futures::stream::StreamExt;

const HEART_RATE_SERVICE: uuid::Uuid = uuid_from_u16(0x180D);
const HEART_RATE_CHARACTERISTICS: uuid::Uuid = uuid_from_u16(0x2A37);

pub struct BluetoothCtl {
    adapters: Vec<Adapter>,
    selected_adapter: Option<Adapter>,
    connected_devices: Vec<String>
}

impl BluetoothCtl {
    pub async fn new() -> Result<BluetoothCtl> {
        let manager = Manager::new().await?;
        let adapters_list = manager.adapters().await?;

        if adapters_list.is_empty() {
            return Err(anyhow!("No adapters found!"));
        }

        let mut adapter: Option<Adapter> = None;
        if adapters_list.len() == 1 {
            adapter = match adapters_list.first() {
                Some(x) => Some(x.to_owned()),
                None => None
            };
        }

        return Ok(
            BluetoothCtl {
                adapters: adapters_list,
                selected_adapter: adapter,
                connected_devices: Vec::new()
            }
        );
    }

    pub async fn select_adapter(&mut self, adapter: Adapter) {
        self.selected_adapter = Some(adapter);
    }

    //https://github.com/deviceplug/btleplug/blob/master/examples/discover_adapters_peripherals.rs
    pub async fn start(&self) -> Result<()> {
        if self.selected_adapter.is_none() {
            return Err(anyhow!("Adapter is not selected!"));

        } else {
            let adapter = self.selected_adapter.as_ref().unwrap();
            let mut events = adapter.events().await?;

            println!("Start scan on {}...", adapter.adapter_info().await?);
            
            adapter.start_scan(ScanFilter::default()).await
                .expect("Can't scan BLE adapter for connected devices...");

            //TODO: or sleep here for 10 seconds and then stop the scan?????

            while let Some(event) = events.next().await {
                match event {
                    CentralEvent::DeviceDiscovered(id) => { // detect only fitness devices have HR
                        // service and power service
                        let peripehral = adapter.peripheral(&id).await.unwrap();
                        
                        if peripehral.properties().await.unwrap().unwrap().local_name.iter().any(|name| name.contains("TICKR")) {
                            println!("Device found");
                            
                            // stop scan test
                            adapter.stop_scan().await?;
                            println!("Stop scanning devices!");

                            peripehral.connect().await?;
                            println!("Connected to device: {:?}", peripehral.properties().await.unwrap().unwrap().local_name);

                            peripehral.discover_services().await?;
                            let charateristics = peripehral.characteristics();
                            println!("Characteristics: {:?}", charateristics);

                                
                            let heart_rate_service = peripehral.services().into_iter().find(|service| service.uuid == HEART_RATE_SERVICE);
                            if heart_rate_service.is_none() {
                                println!("Heart Rate Service not found!");
                                break;
                            }

                            let ch = heart_rate_service.unwrap().characteristics
                                .into_iter()
                                .find(|charateristic| charateristic.properties.contains(CharPropFlags::NOTIFY) 
                                    && charateristic.uuid == HEART_RATE_CHARACTERISTICS);

                            if ch.is_none() {
                                println!("Heart Rate Characteristic not found!");
                                break;
                            }

                            let hr_characteristic = &ch.unwrap();
                            println!("Subscribing to HEART RATE characteristic {:?}", hr_characteristic.uuid);
                            peripehral.subscribe(hr_characteristic).await?;


                            let heart_rate_ch = hr_characteristic.clone();
                            println!("Found response characteristic: {:?}", heart_rate_ch);

                            let mut notification_stream = peripehral.notifications().await?;

                            tokio::spawn(async move {
                                while let Some(notification) = notification_stream.next().await {
                                    if notification.uuid == heart_rate_ch.uuid {
                                        let data = notification.value;

                                        if data.len() < 3 { //TODO: not sure if i need this check
                                            println!("Invalid data - [{}] {:?}", notification.uuid, data);
                                            continue;
                                        }

                                        //TODO: read => https://stackoverflow.com/questions/65443033/heart-rate-value-in-ble/65458794?noredirect=1#comment118474300_65458794
                                        //
                                        // if byte 0 bit[0] is 0 then byte 2 is heart rate else
                                        // byte 2 is u16 heart rate????
                                        let hr = data[1];
                                        println!("Heart rate: {:?}", hr);
                                    } else {
                                        println!("unknown notification: {:?}", notification);
                                    }
                                }
                            });
                            println!("Waiting for notifications");
                        }
                    }
                    CentralEvent::DeviceConnected(id) => {

                    }
                    CentralEvent::DeviceDisconnected(id) => {
                        
                    }
                    _ => {}
                }
            }

            return Ok(());
        }
    }

    pub async fn connect_peripheral(&self, peripheral: &impl Peripheral) -> Result<()> {
        peripheral.connect().await?;
        return Ok(());
    }

    pub async fn disconnect_peripheral(&self, peripheral: &impl Peripheral) -> Result<()> {
        peripheral.disconnect().await?;
        return Ok(());
    }

}


