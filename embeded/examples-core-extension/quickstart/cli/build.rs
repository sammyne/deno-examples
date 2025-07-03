// Copyright 2018-2025 the Deno authors. MIT license.

use std::path::PathBuf;
use std::{env, fs};

use libdeno::core::snapshot::{CreateSnapshotOptions, create_snapshot};

fn main() {
    const FILENAME: &str = std::concat!(env!("CARGO_PKG_NAME"), ".snapshot");

    let options = CreateSnapshotOptions {
        cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
        startup_snapshot: None,
        extensions: vec![core_extension_quickstart_ext::hello_world::init()],
        with_runtime_cb: None,
        skip_op_registration: false,
        extension_transpiler: None,
    };
    let warmup_script = None;

    let snapshot = create_snapshot(options, warmup_script).expect("Error creating snapshot");

    // Save the snapshot for use by our source code:
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let file_path = out_dir.join(FILENAME);
    fs::write(file_path, snapshot.output).expect("Failed to write snapshot");

    // Let cargo know that builds depend on these files:
    for path in snapshot.files_loaded_during_snapshot {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}
