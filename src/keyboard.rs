/// PS/2 Keyboard Driver
/// Reads scan codes directly from hardware port 0x60

const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

/// Read a byte from an x86 I/O port
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!(
        "inb %dx, %al",
        in("dx") port,
        out("al") value,
        options(att_syntax, nostack)
    );
    value
}

/// Wait until the keyboard buffer has data
fn keyboard_has_data() -> bool {
    unsafe { inb(KEYBOARD_STATUS_PORT) & 0x01 != 0 }
}

/// Read one raw scan code from the keyboard (blocking)
pub fn read_scancode() -> u8 {
    while !keyboard_has_data() {}
    unsafe { inb(KEYBOARD_DATA_PORT) }
}

/// Convert a scan code to an ASCII character
/// Returns None for non-printable keys (shift, ctrl, release events etc)
pub fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    // Key release events have bit 7 set — ignore them
    if scancode & 0x80 != 0 {
        return None;
    }

   let table: [u8; 54] = [
    0,     // 0x00 - unused
    0,     // 0x01 - Escape
    b'&', b'e', b'"', b'\'',b'(', b'-', b'e', b'_', b'c', b'a',
    b')', b'=',
    0x08,  // 0x0E - Backspace
    b'\t', // 0x0F - Tab
    b'a', b'z', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p',
    b'^', b'$',
    b'\n', // 0x1C - Enter
    0,     // 0x1D - Left Ctrl
    b'q', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l',
    b'm', b'u', b'`',
    0,     // 0x2A - Left Shift
    b'*',
    b'w', b'x', b'c', b'v', b'b', b'n', b',',
    b';', b':', b'!',
];

    let idx = scancode as usize;
    if idx < table.len() && table[idx] != 0 {
        Some(table[idx])
    } else {
        None
    }
}

/// Read one printable ASCII character (blocking, skips non-printable keys)
pub fn read_char() -> u8 {
    loop {
        let scancode = read_scancode();
        if let Some(ascii) = scancode_to_ascii(scancode) {
            return ascii;
        }
    }
}

pub fn wait_any_key() {
    loop {
        unsafe {
            let status = inb(0x64);
            if status & 0x01 != 0 {
                let _ = inb(0x60);
                return;
            }
        }
    }
}