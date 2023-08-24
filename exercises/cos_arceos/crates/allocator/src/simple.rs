//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    data: [u8; Self::SIZE],
    // point to first free mem addr
    ptr: usize,
    // if num_allocations 1->0, then reset ptr to 0
    num_allocations: usize,
}

impl SimpleByteAllocator {
    const SIZE: usize = 10 * 1024 * 1024;
    pub const fn new() -> Self {
        Self {
            data: [0; Self::SIZE],
            ptr: 0,
            num_allocations: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {}

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Ok(())
    }
}

impl ByteAllocator for SimpleByteAllocator {
    /// See also: slice::align_to
    /// Why return ptr is NonZeroUsize not NonNull?
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        let size = layout.size();
        let align = 2usize.pow(layout.align() as u32);

        let div = size / align;
        let rem = size % align;
        let size = if rem != 0 { div + 1 } else { div } * align;

        if self.ptr + size > Self::SIZE {
            return Err(AllocError::NoMemory);
        }
        let start = self.ptr;
        self.ptr += size;
        self.num_allocations += 1;
        let ptr = self.data[start..self.ptr].as_mut_ptr() as usize;

        Ok(NonZeroUsize::new(ptr).unwrap())
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.num_allocations -= 1;
        if self.num_allocations == 0 {
            self.ptr = 0;
        }
    }

    fn total_bytes(&self) -> usize {
        Self::SIZE
    }

    fn used_bytes(&self) -> usize {
        self.ptr
    }

    fn available_bytes(&self) -> usize {
        Self::SIZE - self.ptr
    }
}

#[cfg(not)]
unsafe impl core::alloc::GlobalAlloc for SimpleByteAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}

/// Allocator is a type parameter in Vec
#[cfg(not)]
unsafe impl core::alloc::Allocator for SimpleByteAllocator {
    fn allocate(
        &self,
        layout: Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        todo!()
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: Layout) {
        todo!()
    }
}
