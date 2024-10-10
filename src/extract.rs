use anyhow::{Context, Result};
use std::path::Path;
use wasmtime::*;

pub(crate) fn extract_from_file<P>(wasm_path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let bytes = std::fs::read(&wasm_path)
        .with_context(|| format!("Failed to read from {:?}", wasm_path.as_ref()))?;
    extract(&bytes)
}

fn extract(bytes: impl AsRef<[u8]>) -> Result<String> {
    let engine = Engine::new(Config::new().wasm_memory64(true))
        .context("Failed to create a Wasmtime Engine")?;
    let mut store = Store::new(&engine, ());
    let mut linker = Linker::new(&engine);
    let module = Module::new(&engine, bytes.as_ref())
        .context("Failed to create a Wasm Module from the bytes")?;
    for import in module.imports() {
        if let ExternType::Func(func) = import.ty() {
            linker
                .func_new(import.module(), import.name(), func, |_, _, _| Ok(()))
                .with_context(|| {
                    format!(
                        "Failed to add mock import for {}::{}",
                        import.module(),
                        import.name()
                    )
                })?;
        }
    }

    let canister = linker
        .instantiate(&mut store, &module)
        .context("Failed to instantiate the Canister Wasm module")?;

    let memory = canister
        .get_memory(&mut store, "memory")
        .context("Failed to find a memory export named 'memory'")?;

    let mut i = if memory.ty(&store).is_64() {
        let get_candid_pointer = canister
            .get_typed_func::<(), i64>(&mut store, "get_candid_pointer")
            .with_context(|| {
                "Failed to find the function export 'get_candid_pointer' which returns i64"
            })?;
        let candid_pointer = get_candid_pointer
            .call(&mut store, ())
            .context("Failed to call the function 'get_candid_pointer'")?;
        candid_pointer as usize
    } else {
        let get_candid_pointer = canister
            .get_typed_func::<(), i32>(&mut store, "get_candid_pointer")
            .with_context(|| {
                "Failed to find the function export 'get_candid_pointer' which returns i32"
            })?;
        let candid_pointer = get_candid_pointer
            .call(&mut store, ())
            .context("Failed to call the function 'get_candid_pointer'")?;
        candid_pointer as usize
    };

    let memory_buffer = memory.data(&store);
    let mut str_vec = vec![];
    while memory_buffer[i] != 0 {
        str_vec.push(memory_buffer[i]);
        i += 1;
    }
    let s = String::from_utf8(str_vec).context("Failed to convert the bytes to a String")?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal() {
        let extracted = extract(
            r#"
(module
  (memory $mem 1)
  (export "memory" (memory $mem))
  (data (i32.const 0) "service : {}")
  (func $get_candid_pointer (result i32)
    i32.const 0
  )
  (export "get_candid_pointer" (func $get_candid_pointer))
)
"#,
        )
        .unwrap();
        assert_eq!(extracted, "service : {}");
    }

    #[test]
    fn import() {
        let extracted = extract(
            r#"
(module
  (import "ic0" "time" (func (result i32)))
  (memory $mem 1)
  (export "memory" (memory $mem))
  (data (i32.const 0) "service : {}")
  (func $get_candid_pointer (result i32)
    i32.const 0
  )
  (export "get_candid_pointer" (func $get_candid_pointer))
)
"#,
        )
        .unwrap();
        assert_eq!(extracted, "service : {}");
    }

    #[test]
    fn non_zero_pointer() {
        let extracted = extract(
            r#"
(module
  (memory $mem 1)
  (export "memory" (memory $mem))
  (data (i32.const 10) "service : {}")
  (func $get_candid_pointer (result i32)
    i32.const 10
  )
  (export "get_candid_pointer" (func $get_candid_pointer))
)
"#,
        )
        .unwrap();
        assert_eq!(extracted, "service : {}");
    }

    #[test]
    fn end_by_0_byte() {
        let extracted = extract(
            r#"
(module
  (memory $mem 1)
  (export "memory" (memory $mem))
  (data (i32.const 0) "service : {}" "\00" "extra text that should be ignored")
  (func $get_candid_pointer (result i32)
    i32.const 0
  )
  (export "get_candid_pointer" (func $get_candid_pointer))
)
"#,
        )
        .unwrap();
        assert_eq!(extracted, "service : {}");
    }

    #[test]
    fn memory64() {
        let extracted = extract(
            r#"
(module
  (memory $mem i64 1)
  (export "memory" (memory $mem))
  (data (i64.const 0) "service : {}")
  (func $get_candid_pointer (result i64)
    i64.const 0
  )
  (export "get_candid_pointer" (func $get_candid_pointer))
)
"#,
        )
        .unwrap();
        assert_eq!(extracted, "service : {}");
    }
}
