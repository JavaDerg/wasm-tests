use wasmer::{imports, Function, Instance, Module, Store, Value, Array};
use std::time::Instant;

fn count() -> u64 {
    10_000_000
}

fn main() {
    let start = Instant::now();

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
    let t = Instant::now() - start;
    println!("setup: {:.4}µs", t.as_secs_f64() * 1000.0);

    // let mem = instance.exports.get_memory("memory").unwrap().clone();

    let num = instance.exports.get_function("busy").unwrap().clone();

    println!("start");

    let mut wasm = 0.0;
    let mut native = 0.0;

    for _ in 0..50 {

        let start = Instant::now();
        let _ = num.call(&[]).unwrap();
        let t = Instant::now() - start;
        println!("  wasm: {:.4}ms", t.as_secs_f64() * 1000.0);
        wasm += t.as_secs_f64();

        let start = Instant::now();
        let _ = nbody::run(count() as usize);
//        let _ = busy();
        let t = Instant::now() - start;
        println!("native: {:.4}ms", t.as_secs_f64() * 1000.0);
        native += t.as_secs_f64();
    }

    println!("  wasm: {:.4}ms", wasm * 1000.0);
    println!("native: {:.4}ms", native * 1000.0);

    println!("native speedup: {:.4}x", wasm / native);
}


#[no_mangle]
pub extern "C" fn busy() -> u64 {
    let mut num = 0u64;
    let count = count();

    loop {
        unsafe { core::ptr::write_volatile(&mut num as *mut u64, num + 1); }
        if num >= count { break num }
    }
}
