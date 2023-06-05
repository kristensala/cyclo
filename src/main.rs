use anyhow::Result;
use bluetoothctl::BluetoothCtl;
use app::Stopwatch;
use iced::Application;

pub mod bluetoothctl;
pub mod app;

#[tokio::main]
async fn main() -> Result<()> {
    let ctl = BluetoothCtl::new().await?;

    ctl.scan().await.unwrap();
    _ = ctl.listen_events().await.unwrap();
    return Ok(());
}
