use btleplug::platform::PeripheralId;

pub enum DeviceType {
    Unknown,
    HeartRateMonitor,
    TurboTrainer
}

#[derive(Debug, Clone)]
pub struct Device {
    pub peripheral_id: PeripheralId,
    pub local_name: String,
    pub is_connected: bool,
}
