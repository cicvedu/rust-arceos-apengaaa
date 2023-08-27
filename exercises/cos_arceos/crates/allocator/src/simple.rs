// //! Simple memory allocation.
// //!
// //! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    start: usize,
    end: usize,
    next: usize,
    allocations: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            next: 0,
            allocations: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.next = start;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        if self.start == 0 {
            self.init(start, size);
            Ok(())
        } else {
            Err(AllocError::InvalidParam)
        }
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        let size = layout.size();
        if self.next + size > self.end {
            return Err(AllocError::NoMemory);
        }
        let result = self.next;
        self.next += size;
        self.allocations += 1;
        NonZeroUsize::new(result).ok_or(AllocError::InvalidParam)
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.next = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.next - self.start
    }

    fn available_bytes(&self) -> usize {
        self.end - self.next
    }
}