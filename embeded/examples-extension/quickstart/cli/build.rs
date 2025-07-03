use libdeno::runtime;
use libdeno::runtime::ops::bootstrap::SnapshotOptions;
use libdeno::x;

fn main() {
    const FILENAME: &str = std::concat!(env!("CARGO_PKG_NAME"), ".snapshot");

    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("get env OUT_DIR"))
        .join(FILENAME);

    let snapshot_options = SnapshotOptions {
        ts_version: x::TS_VERSION.to_owned(),
        v8_version: runtime::deno_core::v8::VERSION_STRING,
        target: std::env::var("TARGET").unwrap(),
    };

    let custom = vec![ext::hello_world::init()];

    runtime::snapshot::create_runtime_snapshot(out, snapshot_options, custom);
}
