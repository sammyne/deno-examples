use std::path::PathBuf;

use clap::Parser;
use libdeno::core::anyhow::Context as _;
use libdeno::core::error::AnyError;
use libdeno::runtime::worker::{MainWorker, WorkerOptions};
use libdeno::x;
use libdeno::x::runtime;
use libdeno::x::url;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let args = Cli::parse();

    let module_url = url::resolve_path(&args.js_path).context("resolve module url")?;

    let service_options = runtime::new_worker_service_options();
    let worker_options = WorkerOptions {
        startup_snapshot: Some(&x::DUMMY_SNAPSHOT),
        ..Default::default()
    };

    let mut worker = MainWorker::bootstrap_from_options(&module_url, service_options, worker_options);
    worker.execute_main_module(&module_url).await?;
    worker.run_event_loop(false).await?;

    Ok(())
}


#[derive(Parser)]
struct Cli {
    /// 待执行的 JS 文件路径。
    #[clap(short, long, default_value = "static/hello-world.js")]
    js_path: PathBuf,
}