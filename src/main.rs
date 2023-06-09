use std::io::stdout;

use anyhow::Result;
use bluetoothctl::BluetoothCtl;
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen}, execute, event::EnableMouseCapture};
use tui::{backend::CrosstermBackend, Terminal};

pub mod bluetoothctl;
pub mod app;
pub mod device;

#[tokio::main]
async fn main() -> Result<()> {
    let mut ctl = BluetoothCtl::new().await?;

    ctl.scan().await.unwrap();
    _ = ctl.listen_events().await.unwrap();
    return Ok(());
}

async fn start_ui() -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    return Ok(());

}
