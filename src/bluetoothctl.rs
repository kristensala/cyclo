use std::borrow::Borrow;
use std::time::Duration;

use anyhow::{Result, anyhow};
use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Manager, Adapter};
use tokio::time;


pub struct BluetoothCtl {
    adapters: Vec<Adapter>,
    selected_adapter: Option<Adapter>
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
                selected_adapter: adapter
            }
        );
    }

    pub async fn select_adapter(&mut self, adapter: Adapter) {
        self.selected_adapter = Some(adapter);
    }

    //https://github.com/deviceplug/btleplug/blob/master/examples/discover_adapters_peripherals.rs
    pub async fn scan_devices(&self) -> Result<()> {
        
        for adapter in self.adapters.iter() {
            println!("Start scan on {}...", adapter.adapter_info().await?);

            adapter.start_scan(ScanFilter::default()).await
                .expect("Cant scan BLE adapter for connected devices...");

            time::sleep(Duration::from_secs(10)).await;

            let peripherals = adapter.peripherals().await?;

            if peripherals.is_empty() {
                return Err(anyhow!("BLE peripehral devices not found."));
            }

            for peripheral in peripherals.iter() {
                let props = peripheral.properties().await?;
                let local_name = props
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(name unknow)"));

                println!("Peripheral: {:?}, ", local_name);
            }
        }

        return Ok(());
    }

    pub async fn connect_peripheral(peripheral_address: String) {
        todo!();
    }

}


