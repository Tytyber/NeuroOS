use bootloader_api::info::{MemoryRegionKind, MemoryRegions};

pub struct BumpAllocator {
    heap_start: u64,
    heap_end: u64,
    next: u64,
}

impl BumpAllocator {
    pub fn new(regions: &MemoryRegions) -> Self {
        for r in regions.iter() {
            if r.kind == MemoryRegionKind::Usable {
                return Self {
                    heap_start: r.start,
                    heap_end: r.end,
                    next: r.start,
                };
            }
        }

        panic!("No usable memory");
    }

    pub fn alloc(&mut self, size: u64, align: u64) -> Option<u64> {
        let mut addr = self.next;

        if addr % align != 0 {
            addr += align - (addr % align);
        }

        if addr + size > self.heap_end {
            return None;
        }

        self.next = addr + size;
        Some(addr)
    }
}