use wasmer::{imports, Function, Instance, Module, Store, Value, Array};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C" fn run_vm() -> u32 {
    let store = Store::default();
    let module = Module::from_binary(
        &store,
        include_bytes!("hw.wasm"),
    )
        .unwrap();

    let import_objects = imports! {
        "env" => {
            // i should have probably removed this
            "ext_num" => Function::new_native(&store, || 0),
        },
    };
    let instance = Instance::new(&module, &import_objects).unwrap();
    let mem = instance.exports.get_memory("memory").unwrap().clone();

    let num = instance.exports.get_function("leak_num").unwrap().clone();

    let loc = num.call(&[]).unwrap()[0].i32().unwrap() as u32;

    let ptr = unsafe { mem.data_ptr().add(loc as usize) } as *mut u32;

    unsafe { *ptr }
}

