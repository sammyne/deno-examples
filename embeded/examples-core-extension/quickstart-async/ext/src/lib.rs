use deno_core::{extension, op2};

#[op2(async)]
async fn greet(#[string] value: String) {
    println!("Received this value from JS: {value}");
}

extension!(
  hello_world,
  ops = [greet],
  // Must specify an entrypoint so that our module gets loaded while snapshotting:
  esm_entry_point = "sammyne:hello_world",
  esm = [
    dir "src",
    "sammyne:hello_world" = "hello_world.js",
  ],
);
