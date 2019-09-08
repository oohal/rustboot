use core::cell::UnsafeCell;
use core::alloc::{GlobalAlloc,Layout};
use core::ptr;
// pilfered from https://doc.rust-lang.org/core/alloc/trait.GlobalAlloc.html
// and hacked to remove the cortex-m depedency

use prlog;

// Bump pointer allocator for *single* core systems
pub struct BumpPointerAlloc {
    pub head: UnsafeCell<usize>,
    pub end: usize,
}

unsafe impl Sync for BumpPointerAlloc {}

unsafe impl GlobalAlloc for BumpPointerAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let head = self.head.get();

            let align = layout.align();
            let size = layout.size();

            let start = (*head + align) & !(align - 1);

            prlog!("start: {:x} align: {:x} size: {:x}", start, align, size);

            if start + size > self.end {
                // a null pointer signal an Out Of Memory condition
                ptr::null_mut()
            } else {
                *head = start + size;
                start as *mut u8
            }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // this allocator never deallocates memory
    }
}

