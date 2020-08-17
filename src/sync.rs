extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use crate::thread::Thread;

pub struct Semaphore {
    value: usize,
    waiters: Vec<u32>,
}

impl Semaphore {
    pub fn new(value: usize) -> Semaphore {
        Semaphore { value, waiters: vec![] }
    }
}

