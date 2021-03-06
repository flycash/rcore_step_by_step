#![feature(lang_items)]
#![feature(asm)]
#![feature(panic_info_message)]
#![feature(global_asm)]
#![no_std]
#![feature(alloc)]
#![feature(naked_functions)]

#[macro_use]
pub mod io;

extern crate alloc;

mod lang_items;
mod context;
mod interrupt;
mod init;
mod clock;
mod memory;
mod consts;
mod process;

use buddy_system_allocator::LockedHeap;
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
