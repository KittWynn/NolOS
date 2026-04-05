/// Graphics Shell — 640x480 32bpp version

use crate::framebuffer::{self, colors};
use crate::font;
use crate::keyboard;
use crate::allocator;

const COLS: usize = 78;
const ROWS: usize = 46;
const CHAR_W: usize = 8;
const CHAR_H: usize = 10;
const X_OFF: usize = 6;
const Y_OFF: usize = 20;

const FG:        u32 = colors::LIGHT_GREEN;
const BG:        u32 = colors::BLACK;
const PROMPT_FG: u32 = colors::CYAN;
const ERR_FG:    u32 = colors::LIGHT_RED;
const INFO_FG:   u32 = colors::YELLOW;

pub struct GfxShell {
    row:    usize,
    col:    usize,
    buf:    [[u8; COLS]; ROWS],
    colors: [[u32; COLS]; ROWS],
}

impl GfxShell {
    pub fn new() -> Self {
        GfxShell {
            row:    0,
            col:    0,
            buf:    [[b' '; COLS]; ROWS],
            colors: [[FG; COLS]; ROWS],
        }
    }

    pub fn init_screen(&mut self) {
        framebuffer::init();
        framebuffer::clear(BG);

        // Title bar
        framebuffer::fill_rect(0, 0, framebuffer::SCREEN_WIDTH, 16, colors::DARK_BLUE);
        framebuffer::fill_rect(0, 15, framebuffer::SCREEN_WIDTH, 1, colors::LIGHT_BLUE);
        font::draw_str(6, 4, b"NolOS v0.6.0  |  bare-metal Rust kernel  |  640x480x32bpp", colors::WHITE, colors::DARK_BLUE);

        // Bottom bar
        framebuffer::fill_rect(0, framebuffer::SCREEN_HEIGHT - 14, framebuffer::SCREEN_WIDTH, 14, colors::DARK_GRAY);
        framebuffer::fill_rect(0, framebuffer::SCREEN_HEIGHT - 14, framebuffer::SCREEN_WIDTH, 1, colors::LIGHT_GRAY);
        font::draw_str(6, framebuffer::SCREEN_HEIGHT - 11, b"NolOS Shell  |  type 'help' for commands", colors::LIGHT_GRAY, colors::DARK_GRAY);

        self.redraw_all();
    }

    fn redraw_all(&self) {
        for row in 0..ROWS {
            self.redraw_row(row);
        }
    }

    fn redraw_row(&self, row: usize) {
        let y = Y_OFF + row * CHAR_H;
        framebuffer::fill_rect(X_OFF, y, COLS * CHAR_W, CHAR_H, BG);
        for col in 0..COLS {
            let ch = self.buf[row][col];
            let fg = self.colors[row][col];
            if ch != b' ' {
                font::draw_char(X_OFF + col * CHAR_W, y, ch, fg, BG);
            }
        }
    }

    fn scroll(&mut self) {
        for row in 0..ROWS - 1 {
            self.buf[row]    = self.buf[row + 1];
            self.colors[row] = self.colors[row + 1];
        }
        self.buf[ROWS - 1]    = [b' '; COLS];
        self.colors[ROWS - 1] = [FG; COLS];
        self.row = ROWS - 1;
        self.col = 0;
        self.redraw_all();
    }

    pub fn newline(&mut self) {
        self.col = 0;
        if self.row + 1 >= ROWS {
            self.scroll();
        } else {
            self.row += 1;
        }
    }

    pub fn write_char(&mut self, ch: u8, color: u32) {
        if ch == b'\n' { self.newline(); return; }
        if self.col >= COLS { self.newline(); }
        self.buf[self.row][self.col]    = ch;
        self.colors[self.row][self.col] = color;
        let x = X_OFF + self.col * CHAR_W;
        let y = Y_OFF + self.row  * CHAR_H;
        font::draw_char(x, y, ch, color, BG);
        self.col += 1;
    }

    pub fn write_str(&mut self, s: &[u8], color: u32) {
        for &ch in s { self.write_char(ch, color); }
    }

    pub fn write_usize(&mut self, mut n: usize, color: u32) {
        if n == 0 { self.write_char(b'0', color); return; }
        let mut tmp = [0u8; 20];
        let mut i = 0;
        while n > 0 { tmp[i] = b'0' + (n % 10) as u8; n /= 10; i += 1; }
        for j in (0..i).rev() { self.write_char(tmp[j], color); }
    }

    pub fn backspace(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            self.buf[self.row][self.col]    = b' ';
            self.colors[self.row][self.col] = FG;
            let x = X_OFF + self.col * CHAR_W;
            let y = Y_OFF + self.row  * CHAR_H;
            font::draw_char(x, y, b' ', FG, BG);
        }
    }

    pub fn prompt(&mut self) {
        self.write_str(b"nolos> ", PROMPT_FG);
    }

    pub fn run(&mut self) -> ! {
        self.write_str(b"Welcome to NolOS! Type 'help' for commands.", INFO_FG);
        self.newline();
        self.newline();
        self.prompt();

        let mut buf = [0u8; 128];
        let mut len = 0usize;

        loop {
            let scancode = keyboard::read_scancode();
            if scancode & 0x80 != 0 { continue; } // ignore key release

            match scancode {
                0x1C => {
                    // Enter
                    self.newline();
                    if len > 0 {
                        self.dispatch(&buf[..len]);
                        len = 0;
                    }
                    self.prompt();
                }
                0x0E => {
                    // Backspace
                    if len > 0 { len -= 1; self.backspace(); }
                }
                _ => {
                    if let Some(ch) = scancode_to_char(scancode) {
                        if len < buf.len() - 1 {
                            buf[len] = ch;
                            len += 1;
                            self.write_char(ch, FG);
                        }
                    }
                }
            }
        }
    }

    fn dispatch(&mut self, cmd: &[u8]) {
        match cmd {
            b"help" => {
                self.write_str(b"Commands:", INFO_FG);
                self.newline();
                self.write_str(b"  help   - This message",     FG); self.newline();
                self.write_str(b"  clear  - Clear screen",     FG); self.newline();
                self.write_str(b"  uname  - Kernel info",      FG); self.newline();
                self.write_str(b"  mem    - Memory stats",     FG); self.newline();
                self.write_str(b"  colors - Color palette",    FG); self.newline();
                self.write_str(b"  halt   - Halt CPU",         FG); self.newline();
            }
            b"clear" => {
                self.buf    = [[b' '; COLS]; ROWS];
                self.colors = [[FG; COLS]; ROWS];
                self.row    = 0;
                self.col    = 0;
                framebuffer::fill_rect(X_OFF, Y_OFF, COLS * CHAR_W, ROWS * CHAR_H, BG);
            }
            b"uname" => {
                self.write_str(b"NolOS 0.6.0 x86 bare-metal Rust 640x480x32bpp", PROMPT_FG);
                self.newline();
            }
            b"mem" => {
                self.write_str(b"Heap: ", INFO_FG);
                self.write_usize(allocator::used_bytes(), FG);
                self.write_str(b" / ", FG);
                self.write_usize(allocator::total_bytes(), FG);
                self.write_str(b" bytes  |  Allocs: ", INFO_FG);
                self.write_usize(allocator::alloc_count(), FG);
                self.newline();
            }
            b"colors" => {
                self.write_str(b"True color palette:", INFO_FG);
                self.newline();
                let palette: &[(u32, &[u8])] = &[
                    (colors::RED,         b"RED    "),
                    (colors::GREEN,       b"GREEN  "),
                    (colors::YELLOW,      b"YELLOW "),
                    (colors::CYAN,        b"CYAN   "),
                    (colors::MAGENTA,     b"MAGENTA"),
                    (colors::ORANGE,      b"ORANGE "),
                    (colors::PURPLE,      b"PURPLE "),
                    (colors::LIGHT_GREEN, b"LGREEN "),
                    (colors::LIGHT_RED,   b"LRED   "),
                    (colors::PINK,        b"PINK   "),
                    (colors::WHITE,       b"WHITE  "),
                ];
                for &(c, name) in palette {
                    self.write_str(b"  ", FG);
                    self.write_str(name, c);
                    self.write_char(b' ', FG);
                }
                self.newline();
            }
            b"halt" => {
                self.write_str(b"Halting CPU...", ERR_FG);
                self.newline();
                loop {}
            }
            _ => {
                self.write_str(b"Unknown command: ", ERR_FG);
                self.write_str(cmd, FG);
                self.newline();
            }
        }
    }
}

fn scancode_to_char(sc: u8) -> Option<u8> {
    let table: &[(u8, u8)] = &[
        (0x02, b'1'), (0x03, b'2'), (0x04, b'3'), (0x05, b'4'),
        (0x06, b'5'), (0x07, b'6'), (0x08, b'7'), (0x09, b'8'),
        (0x0A, b'9'), (0x0B, b'0'),
        (0x10, b'a'), (0x11, b'z'), (0x12, b'e'), (0x13, b'r'),
        (0x14, b't'), (0x15, b'y'), (0x16, b'u'), (0x17, b'i'),
        (0x18, b'o'), (0x19, b'p'),
        (0x1E, b'q'), (0x1F, b's'), (0x20, b'd'), (0x21, b'f'),
        (0x22, b'g'), (0x23, b'h'), (0x24, b'j'), (0x25, b'k'),
        (0x26, b'l'), (0x27, b'm'),
        (0x2C, b'w'), (0x2D, b'x'), (0x2E, b'c'), (0x2F, b'v'),
        (0x30, b'b'), (0x31, b'n'),
        (0x39, b' '),
    ];
    for &(code, ch) in table {
        if code == sc { return Some(ch); }
    }
    None
}
