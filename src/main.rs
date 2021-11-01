use wasmer::{imports, Function, Instance, Module, Store, Value, Array};
use std::time::Instant;

fn count() -> u64 {
    100_000_000
}

fn main() {
    let store = Store::default();
    let module = Module::from_binary(
        &store,
        include_bytes!("../hw/target/wasm32-unknown-unknown/release/hw.wasm"),
    )
        .unwrap();

    let import_objects = imports! {
        "env" => {
            "count" => Function::new_native(&store, count),
        },
    };
    let instance = Instance::new(&module, &import_objects).unwrap();
    // let mem = instance.exports.get_memory("memory").unwrap().clone();

    let num = instance.exports.get_function("busy").unwrap().clone();

    println!("start");

    let start = Instant::now();
    let _ = num.call(&[]).unwrap();
    let t = Instant::now() - start;
    println!("  wasm: {:.4}ms", t.as_secs_f64() * 1000.0);
    let w = t.as_secs_f64();

    let start = Instant::now();
    let _ = busy();
    let t = Instant::now() - start;
    println!("native: {:.4}ms", t.as_secs_f64() * 1000.0);

    println!("native speedup: {:.4}x", w / t.as_secs_f64());
}


#[no_mangle]
pub extern "C" fn busy() -> u64 {
    let mut num = 0u64;
    let count = count();

    loop {
        let read = unsafe { core::ptr::read_volatile(&num as *const u64) };
        unsafe { core::ptr::write_volatile(&mut num as *mut u64, read + 1); }
        if read >= count { break read }
    }
}
