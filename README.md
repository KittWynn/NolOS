# NolOS

> "One can rub in my face his technical knowledge all they want, but mustn't forget who makes them run!"

A bare-metal x86 kernel written in Rust; no standard library, no OS underneath, no abstractions. Just hardware.

## What is NolOS?

NolOS is a graphical kernel built from scratch, running directly on x86 hardware via QEMU. It boots through GRUB Multiboot, initializes a 640×480 32bpp framebuffer via Bochs VBE, and drops into an interactive shell, all without a single line of OS support underneath.

This is not an OS project built on top of something. Every pixel drawn, every keystroke read, every byte allocated is handled manually.

## Features

- 640×480 32bpp graphical framebuffer via Bochs VBE
- Bitmap font renderer (8×8 characters)
- PS/2 keyboard input via raw scancode reading (AZERTY layout)
- Interactive graphical shell with colored output
- Custom bump allocator (1MB heap)
- Title bar and status bar UI chrome
- Boots via GRUB Multiboot on bare metal / QEMU

## Shell Commands

| Command | Description |
|---------|-------------|
| `help` | List available commands |
| `clear` | Clear the screen |
| `uname` | Kernel version and info |
| `mem` | Heap usage and allocation stats |
| `colors` | Display the color palette |
| `halt` | Halt the CPU |

## Building

**Requirements:**
- Rust nightly
- QEMU
- i686 target (`i686-unknown-none`)
- GRUB (for bootable ISO)

## Notice

This project is unfinished and will likely not be continued in the long run. You're welcome to fork it and continue it yourself, but this page will not receive any more updates.
