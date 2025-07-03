use std::path::Path;

use deno_runtime::deno_core::ModuleSpecifier;
use deno_runtime::deno_core::anyhow::{self, Context};
use deno_runtime::deno_core::url::Url;

pub fn resolve_path(p: &Path) -> anyhow::Result<Url> {
    let p = if p.is_absolute() {
        p.to_owned()
    } else {
        std::env::current_dir().context("get current dir")?.join(p)
    };

    let out = ModuleSpecifier::from_file_path(p).map_err(|_| anyhow::anyhow!("load js file"))?;

    Ok(out)
}
