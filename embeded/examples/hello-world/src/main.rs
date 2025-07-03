use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use clap::Parser;
use deno_resolver::npm::{DenoInNpmPackageChecker, NpmResolver};
use deno_runtime::deno_core::anyhow::Context;
use deno_runtime::deno_core::error::AnyError;
use deno_runtime::deno_core::{FsModuleLoader, ModuleSpecifier, anyhow};
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::{MainWorker, WorkerOptions, WorkerServiceOptions};

pub type Sys = sys_traits::impls::RealSys;

#[derive(Parser)]
struct Cli {
    /// 待执行的 JS 文件路径。
    #[clap(short, long, default_value = "static/hello-world.js")]
    js_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let args = Cli::parse();

    let module_url = {
        let p = resolve_path(args.js_path).context("resolve path")?;
        ModuleSpecifier::from_file_path(p).map_err(|_| anyhow::anyhow!("load js file"))?
    };

    let service_options = new_worker_service_options();
    let worker_options = WorkerOptions {
        startup_snapshot: Some(&deno_utils::DUMMY_SNAPSHOT),
        ..Default::default()
    };

    let mut worker = MainWorker::bootstrap_from_options(&module_url, service_options, worker_options);
    worker.execute_main_module(&module_url).await?;
    worker.run_event_loop(false).await?;

    Ok(())
}

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

fn resolve_path(p: PathBuf) -> anyhow::Result<PathBuf> {
    if p.is_absolute() {
        return Ok(p);
    }

    let out = std::env::current_dir().map(|v| v.join(p))?;
    Ok(out)
}
