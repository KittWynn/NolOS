const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH:  usize = 80;
const VGA_BUFFER:    usize = 0xB8000;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Magenta     = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    Pink        = 13,
    Yellow      = 14,
    White       = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(fg: Color, bg: Color) -> Self {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii:      u8,
    color_code: ColorCode,
}

pub struct Writer {
    col:        usize,
    row:        usize,
    color_code: ColorCode,
}

impl Writer {
    pub fn new(color_code: ColorCode) -> Self {
        Writer { col: 0, row: 0, color_code }
    }

    pub fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.write_at(row, col, b' ', ColorCode::new(Color::White, Color::Black));
            }
        }
        self.col = 0;
        self.row = 0;
    }

    pub fn newline(&mut self) {
        self.col = 0;
        if self.row + 1 < BUFFER_HEIGHT {
            self.row += 1;
        } else {
            self.scroll();
        }
    }

    pub fn backspace(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            self.write_at(self.row, self.col, b' ', self.color_code);
        }
    }

    fn scroll(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let offset = (row * BUFFER_WIDTH + col) * 2;
                let ch    = unsafe { *((VGA_BUFFER + offset) as *const u8) };
                let color = unsafe { *((VGA_BUFFER + offset + 1) as *const u8) };
                let dst   = ((row - 1) * BUFFER_WIDTH + col) * 2;
                unsafe {
                    *((VGA_BUFFER + dst) as *mut u8)     = ch;
                    *((VGA_BUFFER + dst + 1) as *mut u8) = color;
                }
            }
        }
        for col in 0..BUFFER_WIDTH {
            self.write_at(BUFFER_HEIGHT - 1, col, b' ', self.color_code);
        }
        self.row = BUFFER_HEIGHT - 1;
    }

    fn write_at(&self, row: usize, col: usize, ascii: u8, color: ColorCode) {
        let offset = (row * BUFFER_WIDTH + col) * 2;
        unsafe {
            *((VGA_BUFFER + offset) as *mut u8)     = ascii;
            *((VGA_BUFFER + offset + 1) as *mut u8) = color.0;
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte  => {
                if self.col >= BUFFER_WIDTH { self.newline(); }
                self.write_at(self.row, self.col, byte, self.color_code);
                self.col += 1;
            }
        }
    }

    pub fn print_centered(&mut self, text: &str) {
        let len = text.len();
        let pad = if len < BUFFER_WIDTH { (BUFFER_WIDTH - len) / 2 } else { 0 };
        for _ in 0..pad { self.write_byte(b' '); }
        for b in text.bytes() { self.write_byte(b); }
        self.newline();
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(b'?'),
            }
        }
        Ok(())
    }
}
