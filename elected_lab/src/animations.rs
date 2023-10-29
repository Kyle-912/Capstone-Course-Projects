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

    pub fn clear(&mut self) {
        for px in self.strip.iter_mut() {
            *px = RGB8::new(0, 0, 0);
        }
    }

    pub fn set(&mut self, color: RGB8) {
        for px in self.strip.iter_mut() {
            *px = color;
        }
    }

    pub fn to_list(&self) -> [RGB8; WIDTH*HEIGHT] {
        self.strip
    }

    pub fn next(&mut self) {
        if self.px_counter <= 10 {
            self.descending = false;
        } else if self.px_counter >= 200 {
            self.descending = true;
        }
        if self.descending == true {
            self.px_counter -= 1;
        } else {
            self.px_counter += 1;
        }

        self.set(RGB8::new(self.px_counter, 0, self.px_counter + 10u8));
    }
}