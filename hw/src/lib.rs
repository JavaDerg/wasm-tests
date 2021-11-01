#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C" fn leak_num() -> *mut u32 {
    Box::leak(Box::new(0xDEADBEEF))
}

