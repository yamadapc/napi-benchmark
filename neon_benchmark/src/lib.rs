use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  Ok(cx.undefined())
}

fn round_trip(mut cx: FunctionContext) -> JsResult<JsValue> {
  let handle: Handle<JsValue> = cx.argument(0)?;
  handle.downcast_or_throw(&mut cx)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("roundTrip", round_trip)?;
    Ok(())
}
