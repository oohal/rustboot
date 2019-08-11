use core::cell::UnsafeCell;
use core::alloc::{GlobalAlloc,Layout};
use core::ptr;
// pilfered from https://doc.rust-lang.org/core/alloc/trait.GlobalAlloc.html
// and hacked to remove the cortex-m depedency

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
            let res = *head % align;
            let start = if res == 0 { *head } else { *head + align - res };
            if start + align > self.end {
                // a null pointer signal an Out Of Memory condition
                ptr::null_mut()
            } else {
                *head = start + align;
                start as *mut u8
            }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // this allocator never deallocates memory
    }
}

