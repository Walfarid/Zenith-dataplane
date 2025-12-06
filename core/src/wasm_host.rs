use wasmtime::{Engine, Linker, Module, Store, Config};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};
use crate::error::Result;
use std::sync::{Arc, Mutex};

pub struct WasmPlugin {
    store: Arc<Mutex<Store<WasiCtx>>>,
    instance: wasmtime::Instance,
    // For MVP, we assume a simple export "process"
}

pub struct WasmHost {
    engine: Engine,
    linker: Linker<WasiCtx>,
}

impl WasmHost {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true); // Enable if using components, but we use core modules + WASI for now for simplicity
        
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        Ok(Self {
            engine,
            linker,
        })
    }

    pub fn load_plugin(&self, wasm_bytes: &[u8]) -> Result<WasmPlugin> {
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .build();
        
        let mut store = Store::new(&self.engine, wasi);
        let module = Module::new(&self.engine, wasm_bytes)?;
        let instance = self.linker.instantiate(&mut store, &module)?;

        Ok(WasmPlugin {
            store: Arc::new(Mutex::new(store)),
            instance,
        })
    }
}

impl WasmPlugin {
    pub fn trigger(&self) -> Result<()> {
        let mut store = self.store.lock().expect("Lock poisoned");
        // Look for a function named "on_event"
        let func = self.instance.get_typed_func::<(), ()>(&mut *store, "on_event");
        
        match func {
            Ok(f) => {
                f.call(&mut *store, ())?;
                Ok(())
            }
            Err(_) => {
                // If not found, maybe it's just a passive plugin
                Ok(())
            }
        }
    }
}
