

use ili9163c;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub enum PinState {
    Low,
    High,
}

pub struct Pin {
    state: Rc<Cell<PinState>>,
}

impl Pin {
    pub fn new(state: Rc<Cell<PinState>>) -> Self {
        Pin { state: state }
    }
}

impl ili9163c::gpio::Pin for Pin {
    fn high(&mut self) {
        self.state.set(PinState::High);
    }

    fn low(&mut self) {
        self.state.set(PinState::Low);
    }
}
