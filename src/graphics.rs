#[derive(Copy, Clone, Debug, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn from(data: u16) -> Pixel {
        let b = (data & 0b0000000001111) as u8;
        let g = ((data & 0b0000111110000) >> 4) as u8;
        let r = ((data & 0b1111000000000) >> 9) as u8;

        Pixel {
            r: r,
            g: g,
            b: b,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Graphics {
    width: usize,
    height: usize,
    gram: Vec<Pixel>,

    pub display: bool,
    pub inverse: bool,
}

impl Graphics {
    pub fn new(width: usize, height: usize) -> Self {
        Graphics {
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
