#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod memory;
mod paging;
mod interrupts;
mod task;
mod allocator;
mod serial;
use serial::{init_serial, print_str, print_hex};
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

use allocator::ALLOCATOR;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init_serial();

    print_str(">>> NeuroOS booting...\n");

    unsafe {
        ALLOCATOR.init(
            memory::BumpAllocator::new(&boot_info.memory_regions)
        );
    }

    print_str("Allocator ready\n");

    let phys_offset = boot_info
        .physical_memory_offset
        .into_option()
        .expect("No phys offset");

    let mut mapper = unsafe { paging::init(phys_offset) };

    let mut frame_allocator = unsafe {
        paging::BootInfoFrameAllocator::init(&boot_info.memory_regions)
    };

    print_str("Paging ready\n");

    interrupts::init_idt();

    unsafe {
        interrupts::PICS.lock().initialize();
        x86_64::instructions::interrupts::enable();
    }

    print_str("Interrupts enabled\n");

    unsafe {
        task::SCHEDULER.with(|s| {
            s.add_task(task::Task { id: 1, stack_pointer: 0 });
            s.add_task(task::Task { id: 2, stack_pointer: 0 });
        });
    }

    loop {
        x86_64::instructions::hlt();
    }
}
