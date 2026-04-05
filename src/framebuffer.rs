/// Framebuffer using Bochs VBE extension
/// 640x480 32bpp true color via QEMU -vga std
/// LFB mapped at 0xFC000000 (confirmed via QEMU PCI BAR0)

pub const SCREEN_WIDTH:  usize = 640;
pub const SCREEN_HEIGHT: usize = 480;
pub const BPP:           usize = 32;

const VBE_DISPI_IOPORT_INDEX: u16 = 0x01CE;
const VBE_DISPI_IOPORT_DATA:  u16 = 0x01CF;

const VBE_DISPI_INDEX_XRES:        u16 = 0x1;
const VBE_DISPI_INDEX_YRES:        u16 = 0x2;
const VBE_DISPI_INDEX_BPP:         u16 = 0x3;
const VBE_DISPI_INDEX_ENABLE:      u16 = 0x4;
const VBE_DISPI_INDEX_VIRT_WIDTH:  u16 = 0x6;
const VBE_DISPI_INDEX_VIRT_HEIGHT: u16 = 0x7;
const VBE_DISPI_INDEX_X_OFFSET:    u16 = 0x8;
const VBE_DISPI_INDEX_Y_OFFSET:    u16 = 0x9;

const VBE_DISPI_ENABLED:     u16 = 0x01;
const VBE_DISPI_LFB_ENABLED: u16 = 0x40;

// Confirmed correct address from QEMU PCI info (BAR0 of VGA device 2)
const LFB_ADDR: usize = 0xFC000000;

unsafe fn outw(port: u16, val: u16) {
    core::arch::asm!(
        "outw %ax, %dx",
        in("ax") val,
        in("dx") port,
        options(att_syntax, nostack)
    );
}

fn vbe_write(index: u16, val: u16) {
    unsafe {
        outw(VBE_DISPI_IOPORT_INDEX, index);
        outw(VBE_DISPI_IOPORT_DATA,  val);
    }
}

/// Initialize 640x480x32bpp via Bochs VBE
pub fn init() {
    vbe_write(VBE_DISPI_INDEX_ENABLE,      0x00);
    vbe_write(VBE_DISPI_INDEX_XRES,        SCREEN_WIDTH  as u16);
    vbe_write(VBE_DISPI_INDEX_YRES,        SCREEN_HEIGHT as u16);
    vbe_write(VBE_DISPI_INDEX_BPP,         BPP           as u16);
    vbe_write(VBE_DISPI_INDEX_VIRT_WIDTH,  SCREEN_WIDTH  as u16);
    vbe_write(VBE_DISPI_INDEX_VIRT_HEIGHT, SCREEN_HEIGHT as u16);
    vbe_write(VBE_DISPI_INDEX_X_OFFSET,    0);
    vbe_write(VBE_DISPI_INDEX_Y_OFFSET,    0);
    vbe_write(VBE_DISPI_INDEX_ENABLE,      VBE_DISPI_ENABLED | VBE_DISPI_LFB_ENABLED);
}

/// Write a 32bpp pixel at (x, y)
#[inline(always)]
pub fn put_pixel(x: usize, y: usize, color: u32) {
    if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT { return; }
    unsafe {
        *((LFB_ADDR + (y * SCREEN_WIDTH + x) * 4) as *mut u32) = color;
    }
}

/// Fill entire screen
pub fn clear(color: u32) {
    unsafe {
        for i in 0..(SCREEN_WIDTH * SCREEN_HEIGHT) {
            *((LFB_ADDR + i * 4) as *mut u32) = color;
        }
    }
}

/// Fill a rectangle
pub fn fill_rect(x: usize, y: usize, w: usize, h: usize, color: u32) {
    for row in y..(y + h).min(SCREEN_HEIGHT) {
        for col in x..(x + w).min(SCREEN_WIDTH) {
            put_pixel(col, row, color);
        }
    }
}

/// Draw a rectangle outline
pub fn draw_rect(x: usize, y: usize, w: usize, h: usize, color: u32) {
    for col in x..(x + w).min(SCREEN_WIDTH) {
        put_pixel(col, y, color);
        put_pixel(col, (y + h - 1).min(SCREEN_HEIGHT - 1), color);
    }
    for row in y..(y + h).min(SCREEN_HEIGHT) {
        put_pixel(x, row, color);
        put_pixel((x + w - 1).min(SCREEN_WIDTH - 1), row, color);
    }
}

/// Horizontal line
pub fn hline(x: usize, y: usize, len: usize, color: u32) {
    for i in 0..len { put_pixel(x + i, y, color); }
}

/// Vertical line
pub fn vline(x: usize, y: usize, len: usize, color: u32) {
    for i in 0..len { put_pixel(x, y + i, color); }
}

/// True color RGB constants (0x00RRGGBB)
pub mod colors {
    pub const BLACK:       u32 = 0x00000000;
    pub const WHITE:       u32 = 0x00FFFFFF;
    pub const RED:         u32 = 0x00FF0000;
    pub const GREEN:       u32 = 0x0000AA00;
    pub const BLUE:        u32 = 0x000000CC;
    pub const CYAN:        u32 = 0x0000CCCC;
    pub const MAGENTA:     u32 = 0x00CC00CC;
    pub const YELLOW:      u32 = 0x00FFFF00;
    pub const ORANGE:      u32 = 0x00FF8800;
    pub const PURPLE:      u32 = 0x008800CC;
    pub const LIGHT_GRAY:  u32 = 0x00CCCCCC;
    pub const DARK_GRAY:   u32 = 0x00333333;
    pub const LIGHT_BLUE:  u32 = 0x004488FF;
    pub const LIGHT_GREEN: u32 = 0x0044FF44;
    pub const LIGHT_RED:   u32 = 0x00FF4444;
    pub const PINK:        u32 = 0x00FF88FF;
    pub const DARK_BLUE:   u32 = 0x00001144;
    pub const BROWN:       u32 = 0x00884400;
}
