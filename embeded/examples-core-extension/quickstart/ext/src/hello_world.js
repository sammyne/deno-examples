// Minimal example, just passes arguments through to Rust:
export function greet(stringValue) {
  const { greet } = Deno.core.ops;
  greet(stringValue);
}
