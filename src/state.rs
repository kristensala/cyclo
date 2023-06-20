#[derive(Clone, Debug)]
pub struct State {
    pub connected_devices: Vec<String>,
    pub heart_rate: u8,
    pub heart_rate_history: Vec<u8>

}

impl State {
    pub fn new() -> State {
        return State { 
            connected_devices: Vec::new(),
            heart_rate: 0,
            heart_rate_history: Vec::new()
        }
    }
}
