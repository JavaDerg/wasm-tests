use wasmer::{imports, Function, Instance, Module, Store, Value, Array};

fn ext_num() -> u32 {
    321
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
            "ext_num" => Function::new_native(&store, ext_num),
        },
    };
    let instance = Instance::new(&module, &import_objects).unwrap();
    let mem = instance.exports.get_memory("memory").unwrap().clone();

    let num = instance.exports.get_function("leak_num").unwrap().clone();

    let loc = num.call(&[]).unwrap()[0].i32().unwrap() as u32;

    println!("{}", loc);
    let ptr = unsafe { mem.data_ptr().add(loc as usize) } as *mut u32;
    println!("{:X}", unsafe { *ptr });
    println!("{}", unsafe { mem.data_unchecked() }.len());
}
