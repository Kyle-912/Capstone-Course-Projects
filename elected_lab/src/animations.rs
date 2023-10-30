use smart_leds::RGB8;
pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;
pub const NUM_PX: usize = WIDTH * HEIGHT;

pub struct Pulse {
    strip: [RGB8; WIDTH * HEIGHT],
    // color: RGB8,
    px_counter: u8,
    descending: bool,
}

impl Pulse {
    pub fn new(color: RGB8) -> Pulse {
        Self {
            strip: [color; WIDTH * HEIGHT],
            // color: color,
            px_counter: 0,
            descending: false,
        }
    }

    // pub fn clear(&mut self) {
    //     for px in self.strip.iter_mut() {
    //         *px = RGB8::new(0, 0, 0);
    //     }
    // }

    pub fn set(&mut self, color: RGB8) {
        for px in self.strip.iter_mut() {
            *px = color;
        }
    }

    pub fn to_list(&self) -> [RGB8; WIDTH * HEIGHT] {
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

pub struct Snake {
    strip: [RGB8; WIDTH * HEIGHT],
    color: RGB8,
    delta: bool,
    row: usize,
    col: usize,
}

impl Snake {
    pub fn new(color: RGB8) -> Snake {
        Self {
            strip: [RGB8::new(0, 0, 0); WIDTH * HEIGHT],
            color: color,
            delta: true,
            row: 0,
            col: 0,
        }
    }

    pub fn set(&mut self) {
        for (idx, px) in self.strip.iter_mut().enumerate() {
            if idx == self.col * WIDTH + self.row {
                *px = self.color;
            } else {
                *px = RGB8::new(0, 0, 0);
            }
        }
    }

    pub fn to_list(&self) -> [RGB8; WIDTH * HEIGHT] {
        self.strip
    }

    pub fn next(&mut self) {
        if self.row == WIDTH - 1 {
            self.delta = false;
            self.col = (self.col + 1) % 8;
        } else if self.row == 0 {
            self.delta = true;
            self.col = (self.col + 1) % 8;
        }
        if self.delta {
            self.row += 1;
        } else {
            self.row -= 1;
        }
        self.set();
    }
}

pub struct Strobe {
    strip: [RGB8; WIDTH * HEIGHT],
    color: RGB8,
    toggle: bool,
}

impl Strobe {
    pub fn new(color: RGB8) -> Strobe {
        Self {
            strip: [RGB8::new(0, 0, 0); WIDTH * HEIGHT],
            color: color,
            toggle: true,
        }
    }

    pub fn set(&mut self) {
        if self.toggle {
            for px in self.strip.iter_mut() {
                *px = self.color;
            }
        } else {
            for px in self.strip.iter_mut() {
                *px = RGB8::new(0, 0, 0);
            }
        }
    }

    pub fn to_list(&self) -> [RGB8; WIDTH * HEIGHT] {
        self.strip
    }

    pub fn next(&mut self) {
        self.set();
        self.toggle = !self.toggle;
    }
}

pub struct Wave {
    strip: [RGB8; WIDTH * HEIGHT],
    color: RGB8,
    row: usize,
}

impl Wave {
    pub fn new(color: RGB8) -> Wave {
        Self {
            strip: [RGB8::new(0, 0, 0); WIDTH * HEIGHT],
            color,
            row: 0,
        }
    }

    pub fn set(&mut self) {
        for idx in self.row * WIDTH..(self.row + 1) * WIDTH {
            self.strip[idx] = self.color;
        }
    }

    pub fn clear(&mut self) {
        for idx in 0..WIDTH * HEIGHT {
            self.strip[idx] = RGB8::new(0, 0, 0);
        }
    }

    pub fn to_list(&self) -> [RGB8; WIDTH * HEIGHT] {
        self.strip
    }

    pub fn next(&mut self) {
        self.clear(); // Clear the entire board
        self.set(); // Set the current row
        self.row = (self.row + 1) % HEIGHT; // Move to the next row
    }
}

