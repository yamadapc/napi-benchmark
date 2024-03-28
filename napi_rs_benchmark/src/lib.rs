use std::collections::HashMap;
use napi::*;
use napi::bindgen_prelude::*;
use napi::sys::napi_value;
use napi_derive::{js_function, module_exports};

#[js_function(0)]
pub fn hello(ctx: CallContext) -> Result<JsUndefined> {
    ctx.env.get_undefined()
}

#[js_function(1)]
pub fn round_trip(ctx: CallContext) -> Result<JsUnknown> {
    Ok(ctx.get::<JsUnknown>(0)?)
}

#[derive(Default)]
struct RustMap {
    internal: HashMap<String, Ref<()>>,
}

impl Drop for RustMap {
    fn drop(&mut self) {
        println!("Drop RustMap");
    }
}

#[js_function(0)]
pub fn create_rust_map(ctx: CallContext) -> Result<JsExternal> {
    let external = External::new(RustMap::default());
    let external: napi_value = unsafe { External::<RustMap>::to_napi_value(ctx.env.raw(), external) }?;
    let js_external = unsafe { JsExternal::from_raw_unchecked(ctx.env.raw(), external) };
    Ok(js_external)
}

#[js_function(3)]
pub fn rust_map_insert(ctx: CallContext) -> Result<JsUndefined> {
    let mut arg0: External<RustMap> = ctx.get(0)?;
    let key: String = ctx.get(1)?;
    let value: JsUnknown = ctx.get(2)?;
    let value = ctx.env.create_reference(value)?;
    let r = arg0.as_mut();
    r.internal.insert(key, value);

    ctx.env.get_undefined()
}

#[js_function(2)]
pub fn rust_map_lookup(ctx: CallContext) -> Result<JsUnknown> {
    let arg0 = ctx.get::<External<RustMap>>(0)?;
    let key = ctx.get::<JsString>(1)?;
    let key = key.into_utf8()?;
    let key = key.as_str()?;
    let r = arg0.as_ref();
    if let Some(value) = r.internal.get(key) {
        let value = ctx.env.get_reference_value(value)?;
        Ok(value)
    } else {
        panic!("Key not found")
    }
}

#[derive(Default)]
struct RustVec {
    internal: Vec<Ref<()>>,
}

impl Drop for RustVec {
    fn drop(&mut self) {
        println!("Drop RustMap");
    }
}

#[js_function(0)]
pub fn create_rust_vec(ctx: CallContext) -> Result<JsExternal> {
    let external = External::new(RustVec::default());
    let external: napi_value = unsafe { External::<RustVec>::to_napi_value(ctx.env.raw(), external) }?;
    let js_external = unsafe { JsExternal::from_raw_unchecked(ctx.env.raw(), external) };
    Ok(js_external)
}

#[js_function(2)]
pub fn rust_vec_insert(ctx: CallContext) -> Result<JsUndefined> {
    let mut arg0: External<RustVec> = ctx.get(0)?;
    let value: JsUnknown = ctx.get(1)?;
    let value = ctx.env.create_reference(value)?;
    let r = arg0.as_mut();
    r.internal.push(value);

    ctx.env.get_undefined()
}

#[js_function(2)]
pub fn rust_vec_lookup(ctx: CallContext) -> Result<JsUnknown> {
    let arg0 = ctx.get::<External<RustVec>>(0)?;
    let key = ctx.get::<JsNumber>(1)?;
    let r = arg0.as_ref();
    if let Some(value) = r.internal.get(key.get_int64()? as usize) {
        let value = ctx.env.get_reference_value(value)?;
        Ok(value)
    } else {
        panic!("Key not found")
    }
}

#[module_exports]
pub fn init(mut exports: JsObject, _env: Env) -> Result<()> {
    exports.create_named_method("hello", hello)?;
    exports.create_named_method("roundTrip", round_trip)?;

    exports.create_named_method("createRustMap", create_rust_map)?;
    exports.create_named_method("rustMapLookup", rust_map_lookup)?;
    exports.create_named_method("rustMapInsert", rust_map_insert)?;

    exports.create_named_method("createRustVec", create_rust_vec)?;
    exports.create_named_method("rustVecLookup", rust_vec_lookup)?;
    exports.create_named_method("rustVecInsert", rust_vec_insert)?;

    Ok(())
}