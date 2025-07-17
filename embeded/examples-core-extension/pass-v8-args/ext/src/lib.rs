use deno_core::error::CoreError;
use deno_core::serde_v8;
use deno_core::v8;
use deno_core::{extension, op2};

#[op2]
#[buffer]
fn return_option(expect_none: bool) -> Option<Vec<u8>> {
    if expect_none { None } else { Some(vec![1, 2, 3, 4]) }
}

#[op2]
fn string_to_bytes<'a>(
    scope: &mut v8::HandleScope<'a>,
    text: v8::Local<'a, v8::Value>,
) -> Result<v8::Local<'a, v8::Uint8Array>, CoreError> {
    let text = v8::Local::<v8::String>::try_from(text).map_err(|_| CoreError::TLA /*("Invalid argument")*/)?;
    let text_str = serde_v8::to_utf8(text, scope);
    let bytes = text_str.into_bytes();
    let len = bytes.len();
    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(bytes).make_shared();
    let buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
    let u8array = v8::Uint8Array::new(scope, buffer, 0, len).unwrap();

    Ok(u8array)
}

#[op2]
pub fn string_from_bytes<'a>(
    scope: &mut v8::HandleScope<'a>,
    #[buffer] zero_copy: &[u8],
) -> Result<v8::Local<'a, v8::String>, CoreError> {
    let buf = &zero_copy;

    match v8::String::new_from_utf8(scope, buf, v8::NewStringType::Normal) {
        Some(text) => Ok(text),
        // None => Err(range_error("string too long")),
        None => Err(CoreError::TLA),
    }
}

extension!(
  hello_world,
  ops = [return_option, string_to_bytes, string_from_bytes],
  // Must specify an entrypoint so that our module gets loaded while snapshotting:
  esm_entry_point = "sammyne:hello_world",
  esm = [
    dir "src",
    "sammyne:hello_world" = "hello_world.js",
  ],
);
