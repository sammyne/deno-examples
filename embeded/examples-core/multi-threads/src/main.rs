use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use deno_core::url::Url;
use deno_core::v8;
use libdeno::core::anyhow::{self, Context};
use libdeno::core::{FsModuleLoader, JsRuntime, RuntimeOptions};
use libdeno::x::url;
use tokio::runtime::Handle;
use tokio::task::LocalSet;
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut hs = Vec::with_capacity(3);
    for _ in 0..hs.capacity() {
        hs.push(tokio::task::spawn_blocking(run));
    }

    for h in hs {
        let _ = h.await.context("join thread")?;
    }

    Ok(())
}

fn run() -> anyhow::Result<()> {
    let module_url = url::resolve_path(Path::new("static/app.js")).context("resolve module url")?;

    let opts = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    };

    let jsrt = JsRuntime::new(opts);

    let local = LocalSet::new();
    local.spawn_local(call(jsrt, module_url));

    Handle::current().block_on(local);

    Ok(())
}

async fn call(mut jsrt: JsRuntime, module_url: Url) -> anyhow::Result<()> {
    let module_id = jsrt.load_main_es_module(&module_url).await?;
    jsrt.mod_evaluate(module_id).await.context("eval module")?;

    let namespace = jsrt.get_module_namespace(module_id).context("get module namespace")?;
    let namespace = namespace.open(jsrt.v8_isolate().as_mut());

    let scope = &mut jsrt.handle_scope();

    // 调用 hello(who: string): void
    let name = v8::String::new(scope, "hello").context("new v8 string 'hello'")?.into();
    let f = namespace
        .get(scope, name)
        .ok_or_else(|| anyhow::anyhow!("miss export 'hello'"))?
        .cast::<v8::Function>();

    let undef = v8::undefined(scope);

    let who = v8::String::new(scope, "alice").context("new v8 string")?.into();

    let args = &[who];

    let mut ticker = time::interval(Duration::from_secs(1));
    loop {
        let _ = ticker.tick().await;
        let _ = f.call(scope, undef.into(), args);
    }
}
