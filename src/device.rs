use btleplug::platform::PeripheralId;

pub enum DeviceType {
    Unknown,
    HeartRateMonitor,
    TurboTrainer
}

pub struct Device {
    pub peripheral_id: PeripheralId,
    pub local_name: String,
    pub is_connected: bool,
    pub device_type: DeviceType
}
