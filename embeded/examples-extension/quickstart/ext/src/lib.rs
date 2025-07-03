use deno_core::{op2,extension};

#[op2(fast)]
fn greet(#[string] value: String) {
    println!("Received this value from JS: {value}");
}

// extension!(
//   hello_world,
//   // Must specify an entrypoint so that our module gets loaded while snapshotting:
// //   esm_entry_point = "sammyne:hello_world",
//   esm_entry_point = "ext:hello_world/hello_world.js",
// //   esm = [
// //     dir "src",
// //     "sammyne:hello_world" = "hello_world.js",
// //   ],
//     ops = [greet],
//   esm = [
//     dir "src",
//     "hello_world.js",
//   ],
// );


extension!(
  hello_world,
  ops = [greet],
  esm_entry_point = "ext:hello_world/hello_world.js",
  esm = ["hello_world.js"],
);
