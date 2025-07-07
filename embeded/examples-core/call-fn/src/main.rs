use std::path::Path;
use std::rc::Rc;

use deno_core::v8;
use libdeno::core::anyhow::{self, Context};
use libdeno::core::{FsModuleLoader, JsRuntime, RuntimeOptions};
use libdeno::x::url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let module_path = Path::new("static/app.js");

    let module_url = url::resolve_path(module_path).context("resolve module url")?;

    let opts = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    };

    let mut jsrt = JsRuntime::new(opts);

    let module_id = jsrt.load_main_es_module(&module_url).await?;
    jsrt.mod_evaluate(module_id).await.context("eval module")?;

    let namespace = jsrt.get_module_namespace(module_id).context("get module namespace")?;
    let namespace = namespace.open(jsrt.v8_isolate().as_mut());

    let scope = &mut jsrt.handle_scope();

    // 调用 world(): void
    {
        let name = v8::String::new(scope, "world").context("new v8 string 'world'")?.into();
        let f = namespace
            .get(scope, name)
            .ok_or_else(|| anyhow::anyhow!("miss export 'world'"))?
            .cast::<v8::Function>();

        let undef = v8::undefined(scope);

        let _ = f.call(scope, undef.into(), &[]);
    }

    // 调用 hello(who: string): void
    {
        let name = v8::String::new(scope, "hello").context("new v8 string 'hello'")?.into();
        let f = namespace
            .get(scope, name)
            .ok_or_else(|| anyhow::anyhow!("miss export 'hello'"))?
            .cast::<v8::Function>();

        let undef = v8::undefined(scope);

        let who = v8::String::new(scope, "sammyne").context("new v8 string")?.into();

        let _ = f.call(scope, undef.into(), &[who]);
    }

    Ok(())
}
