use std::time::Instant;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern "C" {
    pub fn count() -> u64;
}

#[no_mangle]
pub extern "C" fn leak_num() -> *mut u32 {
    Box::leak(Box::new(0xDEADBEEF))
}

#[no_mangle]
pub extern "C" fn busy() -> u64 {
    let mut num = 0u64;
    let count = unsafe { count() };

    loop {
        let read = unsafe { core::ptr::read_volatile(&num as *const u64) };
        unsafe { core::ptr::write_volatile(&mut num as *mut u64, read + 1); }
        if read >= count { break read }
    }
}
