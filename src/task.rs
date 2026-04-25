pub struct Task {
    pub id: usize,
    pub stack_pointer: u64,
}

pub struct Scheduler {
    tasks: [Option<Task>; 8],
    current: usize,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            tasks: [None, None, None, None, None, None, None, None],
            current: 0,
        }
    }

    pub fn add_task(&mut self, t: Task) {
        for slot in self.tasks.iter_mut() {
            if slot.is_none() {
                *slot = Some(t);
                return;
            }
        }
    }

    pub fn next(&mut self) -> Option<&mut Task> {
        self.current = (self.current + 1) % self.tasks.len();
        self.tasks[self.current].as_mut()
    }
}

use core::cell::UnsafeCell;

pub struct GlobalScheduler {
    inner: UnsafeCell<Scheduler>,
}

unsafe impl Sync for GlobalScheduler {}

impl GlobalScheduler {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(Scheduler::new()),
        }
    }

    pub fn with<F: FnOnce(&mut Scheduler)>(&self, f: F) {
        unsafe { f(&mut *self.inner.get()) }
    }
}

pub static SCHEDULER: GlobalScheduler = GlobalScheduler::new();