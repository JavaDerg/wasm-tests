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
    let _ = nbody::run(unsafe { count() } as usize);

    0
    // let mut num = 0u64;
    // let count = unsafe { count() };
    //
    // loop {
    //     unsafe { core::ptr::write_volatile(&mut num as *mut u64, num + 1); }
    //     if num >= count { break num }
    // }
}
