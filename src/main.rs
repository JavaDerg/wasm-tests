use wasmer::{imports, Function, Instance, Module, Store, Value};

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

    let num = instance.exports.get_function("num").unwrap();
    let res = num.call(&[]).unwrap();
    println!("{:?}", res);
}

/*
- crashes dont matter
- total isolation, only provided api can be called
- easy to load and unload
- architecture agnostic
- fast
- native code, no funky scripting languages
 */
