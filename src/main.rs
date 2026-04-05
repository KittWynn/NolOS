#![no_std]
#![no_main]

mod vga;
mod panic;
mod keyboard;
mod allocator;
mod framebuffer;
mod font;
mod gfx_shell;

const MULTIBOOT_MAGIC:    u32 = 0x1BADB002;
const MULTIBOOT_FLAGS:    u32 = 0x0;
const MULTIBOOT_CHECKSUM: u32 = (0u32).wrapping_sub(MULTIBOOT_MAGIC).wrapping_sub(MULTIBOOT_FLAGS);

#[used]
#[link_section = ".multiboot"]
static MULTIBOOT_HEADER: [u32; 3] = [
    MULTIBOOT_MAGIC,
    MULTIBOOT_FLAGS,
    MULTIBOOT_CHECKSUM,
];

const STACK_SIZE: usize = 16384;

#[used]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        "movl ${size} + {stack}, %esp",
        "call {main}",
        "2: hlt",
        "jmp 2b",
        stack = sym STACK,
        size  = const STACK_SIZE,
        main  = sym kernel_main,
        options(noreturn, att_syntax)
    );
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut shell = gfx_shell::GfxShell::new();
    shell.init_screen();
    shell.run();
}
