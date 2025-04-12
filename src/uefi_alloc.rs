use uefi::boot::*;
use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator{
	unsafe fn alloc(&self, layout: Layout) -> *mut u8{
		return allocate_pool(MemoryType::LOADER_DATA, layout.size()).expect("error: couldn't allocate memory").as_ptr();
	}
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout){
		free_pool(core::ptr::NonNull::new(ptr).expect("error: couldn't retrieve pointer for deallocation"));
	}
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;