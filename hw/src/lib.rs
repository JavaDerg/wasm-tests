#![no_std]
use core::panic::PanicInfo;

extern "C" {
    fn ext_num() -> u32;
}

#[no_mangle]
pub extern "C" fn num() -> u32 {
    unsafe { ext_num() }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
