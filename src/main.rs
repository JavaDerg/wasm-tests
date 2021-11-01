use wasmer::{imports, Function, Instance, Module, Store, Value, Array};

fn main() {
    let store = Store::default();
    let module = Module::from_binary(
        &store,
        include_bytes!("../hw/target/wasm32-unknown-unknown/release/hw.wasm"),
    )
        .unwrap();

    let import_objects = imports! {};
    let instance = Instance::new(&module, &import_objects).unwrap();

    let num = instance.exports.get_function("run_vm").unwrap().clone();

    let deadbeef = num.call(&[]).unwrap()[0].i32().unwrap() as u32;

    println!("{}", deadbeef);
}
