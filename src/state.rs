#[derive(Clone, Debug)]
pub struct State {
    pub heart_rate: u8,
    pub heart_rate_history: Vec<u8>

}

impl State {
    pub fn new() -> State {
        return State { heart_rate: 0, heart_rate_history: Vec::new() }
    }
}
