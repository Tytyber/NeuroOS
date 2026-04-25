use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::registers::control::Cr2;

use crate::{print_str, print_hex};
use pic8259::ChainedPics;
use spin::Mutex;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub const PIC_1: u8 = 32;
pub const PIC_2: u8 = PIC_1 + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1, PIC_2) });

pub const TIMER_INTERRUPT: u8 = PIC_1;

pub fn init_idt() {
    unsafe {
        IDT.page_fault.set_handler_fn(page_fault_handler);
        IDT.load();
    }
}
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    print_str("PAGE FAULT!\n");

    let addr = Cr2::read();

    print_str("Address: ");
    print_hex(addr.as_u64());
    print_str("\n");

    print_str("Error: ");
    print_hex(error_code.bits() as u64);
    print_str("\n");

    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn timer(
    _stack: InterruptStackFrame,
) {
    crate::print_str("tick\n");

    unsafe {
        PICS.lock().notify_end_of_interrupt(32);
    }
}