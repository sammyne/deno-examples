use std::path::Path;
use std::rc::Rc;
use std::time::SystemTime;

use ext::MyState;
use libdeno::core::anyhow::{self, Context};
use libdeno::core::{FsModuleLoader, JsRuntime, PollEventLoopOptions, RuntimeOptions};
use libdeno::x::url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let module_path = Path::new("static/app.js");

    let module_url = url::resolve_path(module_path).context("resolve module url")?;

    let now_unix = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).context("get now")?.as_secs();

    let state = MyState{
        now_unix, 
    };

    let opts = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![ext::op2_sample::init(Some(state))],
        ..Default::default()
    };

    let mut js_runtime = JsRuntime::new(opts);

    let module_id = js_runtime.load_main_es_module(&module_url).await?;
    let result = js_runtime.mod_evaluate(module_id);
    js_runtime
        .run_event_loop(PollEventLoopOptions::default())
        .await
        .context("run event-loop")?;
    result.await.context("eval module")
}
