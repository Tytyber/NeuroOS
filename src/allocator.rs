use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

use crate::memory::BumpAllocator;

pub static mut ALLOCATOR: Global = Global::new();

pub struct Global {
    inner: core::cell::UnsafeCell<Option<BumpAllocator>>,
}

unsafe impl Sync for Global {}

impl Global {
    pub const fn new() -> Self {
        Self {
            inner: core::cell::UnsafeCell::new(None),
        }
    }

    pub unsafe fn init(&self, alloc: BumpAllocator) {
        *self.inner.get() = Some(alloc);
    }

    pub fn get(&self) -> Option<&mut BumpAllocator> {
        unsafe { (*self.inner.get()).as_mut() }
    }
}

unsafe impl GlobalAlloc for Global {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if let Some(a) = self.get() {
            if let Some(addr) = a.alloc(layout.size() as u64, layout.align() as u64) {
                return addr as *mut u8;
            }
        }
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}