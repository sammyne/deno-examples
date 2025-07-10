function get(v) {
  return Deno.core.ops.op_get(v);
}

function set(v) {
  Deno.core.ops.op_set(v)
}

globalThis.op2_sample = {
  get,
  set,
  "now": Deno.core.ops.op_now,
};
