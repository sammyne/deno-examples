import { greet as op_greet } from "ext:core/ops";

function greet() {
  op_greet("world");
}

globalThis.Extension = { greet };

// Minimal example, just passes arguments through to Rust:
// export function greet(stringValue) {
//   const { greet } = Deno.core.ops;
//   greet(stringValue);
// }
