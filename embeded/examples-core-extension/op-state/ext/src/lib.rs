use deno_core::{OpState, extension, op2};

#[derive(Clone, Copy)]
pub struct MyState {
    pub now_unix: u64,
}

#[op2]
#[string]
fn op_get(state: &mut OpState) -> String {
    match state.try_borrow::<String>() {
        Some(v) => v.clone(),
        None => "NOT-FOUND".to_owned(),
    }
}

#[op2(fast)]
fn op_set(state: &mut OpState, #[string] v: String) {
    state.put(v);
}

#[op2(fast)]
#[number]
fn op_now(state: &mut OpState) -> u64 {
    match state.try_borrow::<MyState>() {
        Some(v) => v.now_unix,
        None => 0,
    }
}

extension!(
  op2_sample,
  ops = [op_get, op_set, op_now],
  esm_entry_point = "ext:op2_sample/store.js",
  esm = [ dir "src", "store.js" ],
  options = {
    state: Option<MyState>,
  },
  state = |state, c| {
    if let Some(v) = c.state {
      state.put(v);
    }
  },
  docs = "A small example demonstrating op2 usage.",
);
