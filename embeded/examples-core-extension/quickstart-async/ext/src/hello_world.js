// Minimal example, just passes arguments through to Rust:
export async function greet(stringValue) {
  const { greet } = Deno.core.ops;
  await greet(stringValue);
}
