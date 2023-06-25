#[derive(Debug, Clone)]
pub enum MinorDeviceClass {
    NotSupported,
    Unknown,
    Other,
    //Health devices
    Undefined,
    BloodPressureMonitor,
    Thermometer,
    WeighingScale,
    GlucoseMeter,
    PulseOximeter,
    HeartRateMonitor,
    HealthDataDisplay,
}

#[derive(Debug, PartialEq)]
pub enum MajorServiceClass {
    Unknow,
    Audio,
    Rendering,
    Positioning,
    Networking,
    Capturing,
    ObjectTransfer,
    Telephony,
    Information,
    LimitedDiscoverableMode,
    Reserved
}

#[derive(Debug)]
pub enum MajorDeviceClass {
    Miscellaneous,
    Computer,
    Phone,
    LAN,
    AudioVideo,
    Peripheral,
    Imaging,
    Wearable,
    Toy,
    Health,
    Uncategorized,
    Other
}

//bits to look at 7 6 5 4 3 2
//REF: https://www.ampedrftech.com/datasheets/cod_definition.pdf

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub address: String,
    pub is_connected: bool,
    pub minor_device_class: MinorDeviceClass
}

// check whick bits are set to 1
impl MajorServiceClass {
    pub fn get(binary: String) -> Vec<MajorServiceClass> {
        let major_service_class_bits = &binary[..11];
        let set_bits = MajorServiceClass::get_set_bits(major_service_class_bits);
        return set_bits.into_iter()
            .map(|x| MajorServiceClass::map_value(x))
            .collect();
    }

    fn get_set_bits(binary: &str) -> Vec<usize> {
        let default_binary_length = 24;
        let mut set_bits: Vec<usize> = Vec::new();

        for (index, char) in binary.chars().enumerate() {
            if char == '1' {
                let set_bit = default_binary_length - (index + 1);
                set_bits.push(set_bit);
            }
        }

        return set_bits;
    }

    fn map_value(bit: usize) -> Self {
        match bit {
            23 => MajorServiceClass::Information,
            22 => MajorServiceClass::Telephony,
            21 => MajorServiceClass::Audio,
            20 => MajorServiceClass::ObjectTransfer,
            19 => MajorServiceClass::Capturing,
            18 => MajorServiceClass::Rendering,
            17 => MajorServiceClass::Networking,
            16 => MajorServiceClass::Positioning,
            15 => MajorServiceClass::Reserved,
            14 => MajorServiceClass::Reserved,
            13 => MajorServiceClass::LimitedDiscoverableMode,
            _ => MajorServiceClass::Unknow
        }
    }
}


//TODO: bits to look at 12 11 10 9 8
impl MajorDeviceClass {
    pub fn get(binary: &str) -> MajorDeviceClass {
        return MajorDeviceClass::map_value(binary);
    }

    fn map_value(binary: &str) -> Self {
        match binary {
            "00000" => MajorDeviceClass::Miscellaneous,
            "00001" => MajorDeviceClass::Computer,
            "00010" => MajorDeviceClass::Phone,
            "00011" => MajorDeviceClass::LAN,
            "00100" => MajorDeviceClass::AudioVideo,
            "00101" => MajorDeviceClass::Peripheral,
            "00110" => MajorDeviceClass::Imaging,
            "00111" => MajorDeviceClass::Wearable,
            "01000" => MajorDeviceClass::Toy,
            "01001" => MajorDeviceClass::Health,
            "11111" => MajorDeviceClass::Uncategorized,
            _ => MajorDeviceClass::Other
        }
    }
}

impl MinorDeviceClass {
    pub fn get(binary: &str, major_device_class: MajorDeviceClass) -> MinorDeviceClass {
        //todo: get correct binary slice bits 7 to 2
        match major_device_class {
            MajorDeviceClass::Health => {
                match binary {
                    "00000" => MinorDeviceClass::Undefined,
                    "00001" => MinorDeviceClass::BloodPressureMonitor,
                    "00010" => MinorDeviceClass::Thermometer,
                    "00011" => MinorDeviceClass::WeighingScale,
                    "00100" => MinorDeviceClass::GlucoseMeter,
                    "00101" => MinorDeviceClass::PulseOximeter,
                    "00110" => MinorDeviceClass::HeartRateMonitor,
                    "00111" => MinorDeviceClass::HealthDataDisplay,
                    _ => MinorDeviceClass::Other
                }
            }
            _ => MinorDeviceClass::NotSupported
        }
    }
}

impl Device {
    pub fn new(_name: String,
        _address: String,
        _is_connected: bool,
        _device_class: MinorDeviceClass) -> Device {

        todo!();
    }

    /*
    To parse the hexadecimal value 0x00240404, truncate it to 24 bits and rewrite it in binary first: 0010 0100 0000 0100 0000 0100. Then number the bits as follows: (bit 23) (bit 22) (bit 21) … (bit 3) (bit 2) (bit 1) (bit 0)

    The eleven bits 23 through 13 inclusive are the “major service class.” This is set to
    001 0010 0000. In particular, bits 21 and 18 are set. Bit 21 is defined as “Audio” and bit 18 is defined as “Rendering”.

    The five bits 12 through 8 inclusive are the “major device class.” This is set to 0 0100 which is defined as “Audio/Video”.

    The six bits 7 through 2 inclusive are the “minor device class.” This is set to 00 0001. For the major device class “Audio/Video”, this is defined as Wearable Headset Device, which is the kind of audio device this is.

    (The final two bits 1 through 0 inclusive are undefined.)

    The Bluetooth A2DP profile driver btha2dp.sys translates the “Wearable Headset Device” minor class of device to a KSPIN_DESCRIPTOR.Category value of KSNODETYPE_HEADPHONES.

    The Bluetooth Hands-Free profile driver bthhfenum.sys translates “Wearable Headset Device” to KSNODETYPE_HEADSET.
    */
    pub fn get_class(hex_class: String) -> MinorDeviceClass {
        let test = 0x00240404; // wearable headphones
        let binary = format!("{test:024b}");

        let major_service_class = MajorServiceClass::get(binary);

        println!("binary {:?}", major_service_class);
        return MinorDeviceClass::HeartRateMonitor;
    }
}

#[cfg(test)]
mod tests {
    use crate::device::MajorServiceClass;

    #[test]
    fn get_major_service_class() {
        let test = 0x00240404;
        let binary = format!("{test:024b}");

        let major_service_class = MajorServiceClass::get(binary);

        assert!(major_service_class.contains(&MajorServiceClass::Audio));
    }
}
