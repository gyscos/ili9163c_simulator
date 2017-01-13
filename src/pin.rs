use ili9163c;
use gpio_traits::pin::{Output, PinState};
use std::cell::Cell;
use std::rc::Rc;


pub struct Pin {
    state: Rc<Cell<PinState>>,
}

impl Pin {
    pub fn new(state: Rc<Cell<PinState>>) -> Self {
        Pin { state: state }
    }
}

impl Output for Pin {
    fn high(&mut self) {
        self.state.set(PinState::High);
    }

    fn low(&mut self) {
        self.state.set(PinState::Low);
    }
}
