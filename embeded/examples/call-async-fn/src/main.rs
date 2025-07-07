use std::path::PathBuf;

use clap::Parser;
use libdeno::core::anyhow::{self, Context as _};
use libdeno::core::error::CoreError;
use libdeno::core::v8::{Global, Handle, Local, Promise, PromiseState};
use libdeno::core::{JsRuntime, ModuleId, serde_json, serde_v8, v8};
use libdeno::runtime::worker::{MainWorker, WorkerOptions};
use libdeno::x::{self, runtime, url};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Cli { js_path } = Cli::parse();

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
    // 必要步骤，，否则会报 CoreError::Js(JsError{ name: "ReferenceError", ...})
    worker.evaluate_module(module_id).await.context("evaluate module")?;

    call_ok(&mut worker.js_runtime, module_id).await.context("call ok")?;
    call_err(&mut worker.js_runtime, module_id).await.context("call err")?;

    Ok(())
}

#[derive(Parser)]
struct Cli {
    /// 待执行的 JS 文件路径。
    #[clap(short, long, default_value = "static/app.js")]
    js_path: PathBuf,
}

async fn call_err(jsrt: &mut JsRuntime, module_id: ModuleId) -> anyhow::Result<()> {
    let namespace = jsrt.get_module_namespace(module_id).context("get module namespace")?;
    let namespace = namespace.open(jsrt.v8_isolate().as_mut());

    let _p = {
        let scope = &mut jsrt.handle_scope();

        let name = v8::String::new(scope, "world_err")
            .context("fn name as v8 string")?
            .into();
        let f = namespace
            .get(scope, name)
            .ok_or_else(|| anyhow::anyhow!("miss fn"))?
            .cast::<v8::Function>();

        let undef = v8::undefined(scope);

        let args = ["sammyne"]
            .iter()
            .map(|v| serde_v8::to_v8(scope, v))
            .collect::<Result<Vec<_>, _>>()
            .context("convert args to v8 values")?;

        let r = f.call(scope, undef.into(), &args).context("call fn")?;

        let r = r.try_cast::<Promise>().context("expect async function")?;

        Global::new(scope, r)
    };

    match jsrt.run_event_loop(Default::default()).await.expect_err("miss error") {
        // 业务逻辑抛出异常会导致这里捕捉到 CoreError::Js 类型的错误。
        CoreError::Js(err) if err.exception_message.contains("Uncaught (in promise) MyError") => {
            println!("js error: {err:?}");
        }
        err => anyhow::bail!("unexpected error: {err:?}"),
    }

    Ok(())
}

async fn call_ok(jsrt: &mut JsRuntime, module_id: ModuleId) -> anyhow::Result<()> {
    let namespace = jsrt.get_module_namespace(module_id).context("get module namespace")?;
    let namespace = namespace.open(jsrt.v8_isolate().as_mut());

    let p = {
        let scope = &mut jsrt.handle_scope();

        let name = v8::String::new(scope, "world_ok")
            .context("fn name as v8 string")?
            .into();
        let f = namespace
            .get(scope, name)
            .ok_or_else(|| anyhow::anyhow!("miss fn"))?
            .cast::<v8::Function>();

        let undef = v8::undefined(scope);

        let args = ["sammyne"]
            .iter()
            .map(|v| serde_v8::to_v8(scope, v))
            .collect::<Result<Vec<_>, _>>()
            .context("convert args to v8 values")?;

        let r = f.call(scope, undef.into(), &args).context("call fn")?;

        let r = r.try_cast::<Promise>().context("expect async function")?;

        Global::new(scope, r)
    };

    match jsrt.run_event_loop(Default::default()).await {
        Ok(_) => {}
        // 业务逻辑抛出异常会导致这里捕捉到 CoreError::Js 类型的错误。
        Err(err) => anyhow::bail!("run event loop: {err:?}"),
    }

    // 能够继续往下执行必定因为执行过程没有抛出异常，且调用的 async 函数已返回。

    let scope = &mut jsrt.handle_scope();

    let p = Local::new(scope, &p);
    let p = p.open(scope);
    match p.state() {
        PromiseState::Pending | PromiseState::Rejected => anyhow::bail!("unexpected promise state: {:?}", p.state()),
        PromiseState::Fulfilled => {}
    }

    let r = p.result(scope);
    if r.is_undefined() {}

    let v: serde_json::Value = serde_v8::from_v8(scope, r).context("deserialize return-value")?;
    println!("return-value: {v}");

    // TODO：排查等待已完成的 promise 会卡住的原因。
    // let _out = jsrt.resolve(p).await.context("resolve promise")?;

    Ok(())
}
