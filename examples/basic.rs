extern crate ili9163c;
extern crate ili9163c_simulator;

fn main() {
    let mut driver = ili9163c_simulator::Simulator::driver(128, 128);
    let yellow = ili9163c::driver::parse_color(255, 255, 100);
    driver.draw_line((0, 0), (127, 127), yellow);
    for i in 0..8 {
        let i = i * 16;
        let orange = ili9163c::driver::parse_color(255, i as u8 * 2, 100);
        let green =  ili9163c::driver::parse_color(i as u8 * 2, 255, 100);
        driver.draw_line((0, 0), (i, 127), orange);
        driver.draw_line((0, 0), (127, i), green);
    }
    for i in 0..8 {
        let i = i * 16;
        let color = ili9163c::driver::parse_color(i as u8 * 2, i as u8 * 2, 255);
        driver.draw_circle((8 + i, 128 - 8 - i), 7, color);

    }
    loop {
    }
}
