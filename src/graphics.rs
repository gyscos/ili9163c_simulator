use piston_window::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::Cursor;

#[derive(Copy, Clone, Debug, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn from(data: u16) -> Pixel {
        let b = ((data & 0b0000000000011111) << 3) as u8;
        let g = ((data & 0b0000011111100000) >> 3) as u8;
        let r = ((data & 0b1111100000000000) >> 8) as u8;

        Pixel { r: r, g: g, b: b }
    }

    fn toColor(&self) -> types::Color {
        [(self.r as f32) / 255f32,
         (self.g as f32) / 255f32,
         (self.b as f32) / 255f32,
         1.0]
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct GraphicData {
    width: usize,
    height: usize,
    gram: Vec<Pixel>,

    pub display: bool,
    pub inverse: bool,
}

impl GraphicData {
    pub fn new(width: usize, height: usize) -> Self {
        GraphicData {
            width: width,
            height: height,
            gram: vec![Pixel::default(); width * height],
            display: true,
            inverse: false,
        }
    }

    pub fn set(&mut self, point: Point, value: Pixel) {
        let index = point.x + point.y * self.width;
        self.gram[index] = value;
    }
}

pub fn start_graphics(data: Arc<Mutex<GraphicData>>) {
    thread::spawn(|| run_graphics(data));
}

fn run_graphics(data: Arc<Mutex<GraphicData>>) {
    let w = 650;
    let h = 860;
    let mut window: PistonWindow = WindowSettings::new("ili9163c simulator",
                                                       [w, h])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let image_data = include_bytes!("../assets/background.png");
    let img = ::image::load(Cursor::new(&image_data[..]), ::image::PNG).unwrap();
    let texture = Texture::from_image(&mut window.factory,
                                 img.as_rgba8().unwrap(),
                                 &TextureSettings::new())
        .unwrap();

    let offset = Point { x: 70, y: 95 };
    let cellSize = 3;
    let spacing = 1;

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            image(&texture, c.transform, g);

            let data = data.lock().unwrap();
            if !data.display {
                return;
            }

            for (i, &pixel) in data.gram.iter().enumerate() {
                let x = offset.x + (cellSize + spacing) * (i % data.width);
                let y = offset.y + (cellSize + spacing) * (i / data.width);

                rectangle(pixel.toColor(),
                          [x as f64,
                           y as f64,
                           cellSize as f64,
                           cellSize as f64],
                          c.transform,
                          g);
            }
        });
    }
}
