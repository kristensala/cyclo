use anyhow::Result;
use bluetoothctl::BluetoothCtl;

pub mod bluetoothctl;

#[tokio::main]
async fn main() -> Result<()> {
    let ctl = BluetoothCtl::new().await?;
    ctl.scan_devices().await?;

    return Ok(());
}
