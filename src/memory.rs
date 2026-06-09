const PAGE_SIZE: usize = 4096;
 
#[derive(Clone, Copy)]
pub struct Frame {
    pub start_address: usize,
}

pub struct FrameAllocator {
    pub next: usize,
    pub end: usize,
}

impl FrameAllocator  {
    pub const fn new(start: usize, end: usize) -> Self {
        Self {
            next: start,
            end,
        }
    }
    pub fn alloc(&mut self) -> Option<Frame> {
        if self.next >= self.end {
            return None;
        }

        let frame = Frame {
            start_address: self.next,
        };

        self.next += PAGE_SIZE;
        Some(frame)
    }
}