use wasmer::{imports, Function, Instance, Module, Store, Value, Array, import_namespace};
use std::time::Instant;
use wasmer_wasi::{WasiEnv, WasiState, WasiVersion};
use bytes::BufMut;
use std::io::Write;
use rayon::prelude::*;

fn main() {
    let start = Instant::now();

    let store = Store::default();
    let module = Module::from_binary(
        &store,
        include_bytes!("../hw/target/wasm32-wasi/release/hw.wasm"),
    )
        .unwrap();

    let mut imports = WasiState::new("space_pew_pew").finalize().unwrap().import_object(&module).unwrap();

    let instance = Instance::new(&module, &imports).unwrap();
    let t = Instant::now() - start;
    println!("setup: {:.4}µs", t.as_secs_f64() * 1000.0);

    // let mem = instance.exports.get_memory("memory").unwrap().clone();


    let elapse = instance.exports.get_function("elapse").unwrap().native::<(), u64>().unwrap();


    // let mut max = u64::MIN;
    // let mut min = u64::MAX;
    // let mut avg = 0;

    //
    // for i in 0..10_000_000 {
    //     let t = elapse.call().unwrap();
    //     max = t.max(max);
    //     min = t.min(min);
    //     avg += t;
    //
    //     samples.put_u32_le(t as u32);
    //
    //     if i % 100_000 == 0 {
    //         eprintln!("{}%", i / 100_000);
    //     }
    // }
    // eprintln!("min: {:.3}µs\navg: {:.3}µs\nmax: {:.3}µs", min as f64 / 1000.0, avg as f64 / 10_000_000_000.0, max as f64 / 1000.0);

    let mut samples = bytes::BytesMut::with_capacity(40_000_000);
    (0..10_000_000)
        .into_par_iter()
        .map(|i| {
            let s = Instant::now();
            let _ = elapse.call();
            let ct = s.elapsed().as_nanos() as u32;
            if i % 10000 == 0 {
                eprintln!("hecc {}", i);
            }
            ct
        })
        .for_each(|t| samples.put_u32_le(t));
    std::fs::File::create("samples_u32_le.bin").unwrap().write_all(&samples).unwrap();
}
