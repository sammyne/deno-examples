use std::rc::Rc;
use std::sync::Arc;

use deno_resolver::npm::{DenoInNpmPackageChecker, NpmResolver};
use deno_runtime::deno_core::FsModuleLoader;
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::WorkerServiceOptions;

pub type Sys = sys_traits::impls::RealSys;

pub fn new_worker_service_options() -> WorkerServiceOptions<DenoInNpmPackageChecker, NpmResolver<Sys>, Sys> {
    let fs = Arc::new(RealFs);
    let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(Sys {}));
    let permissions = PermissionsContainer::allow_all(permission_desc_parser);

    WorkerServiceOptions::<DenoInNpmPackageChecker, NpmResolver<Sys>, Sys> {
        blob_store: Default::default(),
        broadcast_channel: Default::default(),
        deno_rt_native_addon_loader: None,
        feature_checker: Default::default(),
        fs,
        module_loader: Rc::new(FsModuleLoader),
        node_services: Default::default(),
        npm_process_state_provider: Default::default(),
        permissions,
        root_cert_store_provider: Default::default(),
        fetch_dns_resolver: Default::default(),
        shared_array_buffer_store: Default::default(),
        compiled_wasm_module_store: Default::default(),
        v8_code_cache: Default::default(),
    }
}
