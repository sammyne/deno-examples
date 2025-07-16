use std::path::Path;
use std::rc::Rc;

use libdeno::core::anyhow::{self, Context};
use libdeno::core::url::Url;
use libdeno::core::{FsModuleLoader, JsRuntime, PollEventLoopOptions, RuntimeOptions};
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let code = fs::read_to_string("static/app.js").await.context("read app.js")?;

    let module_url = Url::from_file_path(Path::new("/app.js")).map_err(|_| anyhow::anyhow!("convert path to URL"))?;

    let opts = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    };

    let mut jsrt = JsRuntime::new(opts);

    // let module_id = js_runtime.load_main_es_module(&module_url).await?;
    let module_id = jsrt
        .load_main_es_module_from_code(&module_url, code)
        .await
        .context("load code")?;
    let result = jsrt.mod_evaluate(module_id);
    jsrt.run_event_loop(PollEventLoopOptions::default())
        .await
        .context("run event-loop")?;
    result.await.context("eval module")
}
