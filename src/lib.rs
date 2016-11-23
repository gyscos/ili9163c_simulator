use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

extern crate ili9163c;

mod pin;
mod graphics;

use graphics::Pixel;
use pin::{Pin, PinState};

pub struct Simulator {
    graphics: Arc<Mutex<graphics::Graphics>>,

    dcx: Rc<Cell<PinState>>,
    csx: Rc<Cell<PinState>>,

    command: u8,
    data_buffer: Vec<u8>,

    cursor: graphics::Point,
    xWindow: (usize, usize),
    yWindow: (usize, usize),

    mirrorX: bool,
    mirrorY: bool,

    xySwitch: bool,
    scanOrderSwitch: bool,
}

fn merge(msb: u8, lsb: u8) -> u16 {
    ((msb as u16) << 8) | lsb as u16
}

impl Simulator {
    fn new(width: usize, height: usize) -> Self {
        let graphics = graphics::Graphics::new(width, height);
        Simulator {
            graphics: Arc::new(Mutex::new(graphics)),
            dcx: Rc::new(Cell::new(PinState::Low)),
            csx: Rc::new(Cell::new(PinState::Low)),
            command: 0x00,
            data_buffer: Vec::new(),
            cursor: graphics::Point::default(),
            xWindow: (0, width - 1),
            yWindow: (0, height - 1),
            mirrorX: false,
            mirrorY: false,
            xySwitch: false,
            scanOrderSwitch: false,
        }
    }

    fn set_command(&mut self, command: u8) {

        self.command = command;
        self.data_buffer.clear();
        // Commands without parameters are handled here.
        match self.command {
            // 20h: INVOFF: Display Inversion Off
            0x20 => self.graphics.lock().unwrap().inverse = false,

            // 21h: INVON: Display Inversion On
            0x21 => self.graphics.lock().unwrap().inverse = true,

            // 28h: DISPOFF - Display Off
            0x28 => self.graphics.lock().unwrap().display = false,

            // 29h: DISPON - Displan On
            0x29 => self.graphics.lock().unwrap().display = true,

            _ => (),
        }
    }

    fn add_data(&mut self, data: u8) {
        // Commands with parameters are handled here.
        match self.command {

            // 2Ah: CASET - Column Address Set
            0x2A => {
                if self.data_buffer.len() < 3 {
                    self.data_buffer.push(data);
                } else {
                    let end_msb = self.data_buffer.pop().unwrap();
                    let end_lsb = data;
                    let end = merge(end_msb, end_lsb) as usize;

                    let start_lsb = self.data_buffer.pop().unwrap();
                    let start_msb = self.data_buffer.pop().unwrap();
                    let start = merge(start_msb, start_lsb) as usize;

                    self.xWindow = (start, end);
                }
            }

            // 2Bh: PASET - Page Address Set
            0x2B => {
                if self.data_buffer.len() < 3 {
                    self.data_buffer.push(data);
                } else {
                    let end_msb = self.data_buffer.pop().unwrap();
                    let end_lsb = data;
                    let end = merge(end_msb, end_lsb) as usize;

                    let start_lsb = self.data_buffer.pop().unwrap();
                    let start_msb = self.data_buffer.pop().unwrap();
                    let start = merge(start_msb, start_lsb) as usize;

                    self.yWindow = (start, end);
                }
            }

            // 2Ch: RAMWR - Memory Write
            0x2C => {
                match self.data_buffer.pop() {
                    None => self.data_buffer.push(data),
                    Some(msb) => {
                        // Combine previous MSB with the latest data (LSB)
                        let data = merge(msb, data);

                        self.graphics
                            .lock()
                            .unwrap()
                            .set(self.cursor, Pixel::from(data));

                        // Advance cursor
                        self.cursor.x += 1;
                        if self.cursor.x == self.xWindow.1 {
                            self.cursor.x = self.xWindow.0;
                            self.cursor.y += 1;
                            if self.cursor.y == self.yWindow.1 {
                                self.cursor.y = self.yWindow.0;
                            }
                        }
                    }
                }
            }

            // 36h: MADCTL - Memory Access Control
            0x36 => {
                self.mirrorY = (data & 0b10000000) != 0;
                self.mirrorX = (data & 0b01000000) != 0;
                self.xySwitch = (data & 0b00100000) != 0;
                self.scanOrderSwitch = (data & 0b00010000) != 0;

                // Ignore RGB/BGR for now
            }

            _ => (),
        }
    }

    /// Returns a new driver connected to a simulator.
    ///
    /// Simulator has the given dimensions.
    pub fn driver(width: usize, height: usize)
                  -> ili9163c::driver::Driver<Self, Pin, Pin> {
        let simulator = Simulator::new(width, height);
        let dcx = Pin::new(simulator.dcx.clone());
        let csx = Pin::new(simulator.csx.clone());

        ili9163c::driver::Driver::new(simulator, dcx, csx)
    }
}

impl ili9163c::spi::Serial for Simulator {
    fn write(&mut self, data: u8) {
        if let PinState::High = self.csx.get() {
            return;
        }

        match self.dcx.get() {
            PinState::Low => self.set_command(data),
            PinState::High => self.add_data(data),
        }
    }
}
