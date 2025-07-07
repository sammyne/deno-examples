use std::path::Path;

use libdeno::core::anyhow::{self, Context};
use libdeno::runtime::worker::{MainWorker, WorkerOptions};
use libdeno::x::{self, runtime, url};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let module_path = Path::new("static/app.js");

    let module_url = url::resolve_path(module_path).context("resolve module url")?;

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
