#[derive(Clone, Copy, Debug)]
pub struct State {
    pub heart_rate: u8
}

impl State {
    pub fn new() -> State {
        return State { heart_rate: 0 }
    }
}
