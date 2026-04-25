use x86_64::{
    VirtAddr,
    structures::paging::{
        OffsetPageTable, PageTable, PhysFrame, Page, Size4KiB,
        FrameAllocator, Mapper,
    },
    registers::control::Cr3,
};

pub unsafe fn init(offset: u64) -> OffsetPageTable<'static> {
    let level_4 = active_level_4(offset);
    OffsetPageTable::new(level_4, VirtAddr::new(offset))
}

unsafe fn active_level_4(offset: u64) -> &'static mut PageTable {
    let (frame, _) = Cr3::read();
    let phys = frame.start_address();
    let virt = VirtAddr::new(offset) + phys.as_u64();

    &mut *(virt.as_mut_ptr())
}

pub struct BootInfoFrameAllocator;

impl BootInfoFrameAllocator {
    pub unsafe fn init(_regions: &bootloader_api::info::MemoryRegions) -> Self {
        Self
    }
}