pub enum DeviceClass {
    HeartRateMonitor
}

#[derive(Debug)]
pub enum MajorServiceClass {
    Unknow,
    Audio,
    Rendering
}

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub address: String,
    pub is_connected: bool
}

impl MajorServiceClass {
    pub fn get(set_bits: Vec<usize>) -> Vec<MajorServiceClass> {
        return set_bits.into_iter()
            .map(|x| MajorServiceClass::map_value(x))
            .collect();
    }

    fn map_value(bit: usize) -> Self {
        match bit {
            21 => MajorServiceClass::Audio,
            18 => MajorServiceClass::Rendering,
            _ => MajorServiceClass::Unknow
        }
    }
}

impl Device {
    pub fn new() -> Device {
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
    pub fn get_class(hex_class: String) -> DeviceClass {
        let test = 0x00240404;
        let binary = format!("{test:024b}");

        let major_service_class_binary = &binary[..11];
        let set_bits = Device::get_set_bits(major_service_class_binary);

        let class = MajorServiceClass::get(set_bits);
        println!("binary {:?}", class);
        return DeviceClass::HeartRateMonitor;
    }

    fn get_major_service_class(binary: String) {
        let major_service_class_binary = &binary[..11];
        todo!();
    }

    fn get_major_device_class() {
        todo!();
    }

    fn get_minor_device_class() {
        todo!();
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
}


