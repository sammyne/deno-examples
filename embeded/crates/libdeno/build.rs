use deno_runtime::ops::bootstrap::SnapshotOptions;

// 参考 deno/cli/snapshot/build.rs
fn main() {
    let out =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("get env OUT_DIR")).join("DUMMY_DENO.snapshot");

    let snapshot_options = SnapshotOptions {
        ts_version: "5.8.3".to_owned(),
        v8_version: deno_runtime::deno_core::v8::VERSION_STRING,
        target: std::env::var("TARGET").unwrap(),
    };

    deno_runtime::snapshot::create_runtime_snapshot(out, snapshot_options, vec![]);
}
