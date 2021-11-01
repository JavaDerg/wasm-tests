use wasmi::{ModuleInstance, ImportsBuilder, NopExternals, RuntimeValue, ModuleImportResolver, MemoryInstance};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C" fn run_vm() -> u32 {
    let module = wasmi::Module::from_buffer(include_bytes!("hw.wasm")).unwrap();

    let instance =
        ModuleInstance::new(
            &module,
            &ImportsBuilder::default()
        )
            .expect("failed to instantiate wasm module")
            .assert_no_start();

     let pos = match instance.invoke_export("leak_num", &[], &mut NopExternals).unwrap().unwrap() {
        RuntimeValue::I32(pos) => pos as u32,
        _ => panic!(),
    };

    pos
}

