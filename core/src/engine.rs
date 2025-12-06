use crate::ring_buffer::ZenithRingBuffer;
// use crate::event::ZenithEvent;
use crate::wasm_host::WasmHost;
use crate::error::Result;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct ZenithEngine {
    buffer: ZenithRingBuffer,
    wasm_host: Arc<WasmHost>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl ZenithEngine {
    pub fn new(buffer_size: usize) -> Result<Self> {
        Ok(Self {
            buffer: ZenithRingBuffer::new(buffer_size),
            wasm_host: Arc::new(WasmHost::new()?),
            running: Arc::new(std::sync::atomic::AtomicBool::new(true)),
        })
    }

    pub fn get_ring_buffer(&self) -> ZenithRingBuffer {
        self.buffer.clone()
    }

    pub fn start(&self) {
        let buffer = self.buffer.clone();
        let running = self.running.clone();
        // let host = self.wasm_host.clone(); 

        thread::spawn(move || {
            println!("Zenith Core Engine: Consumer thread started.");
            while running.load(std::sync::atomic::Ordering::Relaxed) {
                if let Some(_event) = buffer.pop() {
                    // Process event
                    // In a real implementation, we would pass this to WASM plugins
                    // For now, we just log trace
                    // println!("Processing event seq: {}", event.header.seq_no);
                } else {
                    thread::park_timeout(Duration::from_micros(10));
                }
            }
        });
    }

    pub fn shutdown(&self) {
        self.running.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}
