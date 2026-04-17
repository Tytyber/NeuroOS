#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

// Регистрируем точку входа
entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    init_serial();
    print_str(">>> MyOS Kernel booted successfully!\r\n");
    print_str(">>> AI Policy Hook will be added next.\r\n");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_str("KERNEL PANIC!\r\n");
    loop {}
}

// === Serial Output (COM1 @ 0x3F8) ===
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("outb %al, %dx", in("al") val, in("dx") port);
}

fn init_serial() {
    unsafe {
        outb(0x3F8 + 1, 0x00);
        outb(0x3F8 + 3, 0x80);
        outb(0x3F8 + 0, 0x03);
        outb(0x3F8 + 1, 0x00);
        outb(0x3F8 + 3, 0x03);
        outb(0x3F8 + 2, 0xC7);
        outb(0x3F8 + 4, 0x0B);
    }
}

fn print_char(byte: u8) {
    unsafe {
        while (core::arch::asm!("inb %dx", out("al") -> u8, in("dx") 0x3F8 + 5) & 0x20) == 0 {}
        outb(0x3F8, byte);
    }
}

fn print_str(s: &str) {
    for &byte in s.as_bytes() {
        print_char(byte);
    }
}