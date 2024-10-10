# candid-extractor

A CLI tool to extract the Candid interface from a Canister WASM.

## Installation

```
cargo install candid-extractor
```

You can also use `cargo-binstall` if it's available.

```
cargo binstall candid-extractor
```

## Usage

```
candid-extractor path/to/canister.wasm
```

The Candid interface will be printed to stdout.

## Prerequisites for Canisters

`candid-extractor` can extract the Candid interface if the Canister meets the following requirements:

* Exports a memory named "memory".
* Exports a function named "get_candid_pointer", which returns an index within the "memory".
  * If "memory" is 32-bit, the function should return an `i32`.
  * If "memory" is 64-bit, the function should return an `i64`.
* The Candid interface should be encoded in UTF-8 and stored in "memory" starting from the returned index.
* A "NUL terminator" (byte 0x00) should be added at the end of the data if additional content exists after it in "memory".
