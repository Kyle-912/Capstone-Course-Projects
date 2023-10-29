use smart_leds::RGB8;
pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;
pub const NUM_PX: usize = WIDTH*HEIGHT;

pub struct MODE {
    strip: [RGB8; WIDTH*HEIGHT],
    color: RGB8,
    px_counter: u8,
    descending: bool,
}

impl MODE {
    pub fn new(color: RGB8) -> MODE {
        Self {
            strip: [RGB8::new(0, 0, 0); WIDTH*HEIGHT],
            color: color,
            px_counter: 0,
            descending: false,
        }
    }
}