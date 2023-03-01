use std::sync::Arc;

use containerd_shim_wasm::sandbox::EngineGetter;
use containerd_shim_wasm::sandbox::{Local, ManagerService};
use containerd_shim_wasm::services::sandbox_ttrpc::{create_manager, Manager};
use log::info;
use containerd_shim_wasmedge::instance::Wasi as WasiInstance;
use ttrpc::{self, Server};

fn main() {
    info!("starting up!");
    let s: ManagerService<_, Local<WasiInstance, _>> =
        ManagerService::new(WasiInstance::new_engine().unwrap());
    let s = Arc::new(Box::new(s) as Box<dyn Manager + Send + Sync>);
    let service = create_manager(s);

    let mut server = Server::new()
        .bind("unix:///run/io.containerd.wasmwasi.v1/manager.sock")
        .unwrap()
        .register_service(service);

    server.start().unwrap();
    info!("server started!");
    let (_tx, rx) = std::sync::mpsc::channel::<()>();
    rx.recv().unwrap();
}
