use std::path::PathBuf;

use clap::Parser;
use libdeno::core::anyhow::{self, Context as _};
use libdeno::core::{JsRuntime, ModuleId, serde_json, serde_v8, v8};
use libdeno::runtime::worker::{MainWorker, WorkerOptions};
use libdeno::x::{self, runtime};
use libdeno::x::url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Cli { js_path, func, args } = Cli::parse();

    let module_url = url::resolve_path(&js_path).context("resolve module url")?;

    let service_options = runtime::new_worker_service_options();
    let worker_options = WorkerOptions {
        startup_snapshot: Some(&x::DUMMY_SNAPSHOT),
        ..Default::default()
    };

    let mut worker = MainWorker::bootstrap_from_options(&module_url, service_options, worker_options);

    let module_id = worker
        .preload_main_module(&module_url)
        .await
        .context("load main module")?;

    call(&mut worker.js_runtime, module_id, &func, &args).context("call")?;

    Ok(())
}

#[derive(Parser)]
struct Cli {
    /// 待执行的 JS 文件路径。
    #[clap(short, long, default_value = "static/app.js")]
    js_path: PathBuf,
    /// 函数名称
    #[clap(short, long)]
    func: String,
    // 参数列表
    #[clap(long)]
    args: Vec<serde_json::Value>,
}

fn call(jsrt: &mut JsRuntime, module_id: ModuleId, func: &str, args: &[serde_json::Value]) -> anyhow::Result<()> {
    let namespace = jsrt.get_module_namespace(module_id).context("get module namespace")?;
    let namespace = namespace.open(jsrt.v8_isolate().as_mut());

    let scope = &mut jsrt.handle_scope();

    let name = v8::String::new(scope, func).context("fn name as v8 string")?.into();
    let f = namespace
        .get(scope, name)
        .ok_or_else(|| anyhow::anyhow!("miss fn"))?
        .cast::<v8::Function>();

    let undef = v8::undefined(scope);

    let args = args
        .iter()
        .map(|v| serde_v8::to_v8(scope, v))
        .collect::<Result<Vec<_>, _>>()
        .context("convert args to v8 values")?;

    let _ = f.call(scope, undef.into(), &args);

    Ok(())
}
