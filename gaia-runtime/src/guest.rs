//! A real wasm guest. Adds two numbers. No network.

use anyhow::{anyhow, Result};
use wasmtime::{Engine, Instance, Module, Store};

/// (func (export "add") (param i32 i32) (result i32) local.get 0 local.get 1 i32.add)
const ADD_WASM: &[u8] = &[
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01,
    0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 0x61, 0x64, 0x64, 0x00, 0x00, 0x0a, 0x09,
    0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b,
];

pub fn add(left: i32, right: i32) -> Result<i32> {
    let engine = Engine::default();
    let module = Module::new(&engine, ADD_WASM).map_err(|err| anyhow!("guest did not compile: {err}"))?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).map_err(|err| anyhow!("guest did not start: {err}"))?;
    let add = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "add")
        .map_err(|err| anyhow!("guest has no add: {err}"))?;
    add.call(&mut store, (left, right)).map_err(|err| anyhow!("guest call failed: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_adds_two_numbers() {
        assert_eq!(add(20, 22).unwrap(), 42);
    }
}
