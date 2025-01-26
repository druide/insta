<div align="center">
 <img src="https://github.com/mitsuhiko/insta/blob/master/assets/logo.png?raw=true" width="250" height="250">
 <p><strong>insta: a snapshot testing library for Rust</strong></p>
</div>

[![Build Status](https://github.com/mitsuhiko/insta/workflows/Tests/badge.svg?branch=master)](https://github.com/mitsuhiko/insta/actions?query=workflow%3ATests)
[![Crates.io](https://img.shields.io/crates/d/insta.svg)](https://crates.io/crates/insta)
[![License](https://img.shields.io/github/license/mitsuhiko/insta)](https://github.com/mitsuhiko/insta/blob/master/LICENSE)
[![rustc 1.51.0](https://img.shields.io/badge/rust-1.51.0%2B-orange.svg)](https://img.shields.io/badge/rust-1.51.0%2B-orange.svg)
[![Documentation](https://docs.rs/insta/badge.svg)](https://docs.rs/insta)
[![VSCode Extension](https://img.shields.io/visual-studio-marketplace/v/mitsuhiko.insta?label=vscode%20extension)](https://marketplace.visualstudio.com/items?itemName=mitsuhiko.insta)

## Fork

This is a forked version of the original library which allows to run snapshot
tests for the `WASM` build targets, in the `WasmEdge` or directly in the browser.

1. Install [WasmEdge](https://wasmedge.org/).

2. To run tests in the browser follow [wasm-bindgen-test](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html) instructions.

3. Install `cargo-insta`:
   
   ```bash
   cargo install --git https://github.com/druide/insta.git cargo-insta --locked
   ```

4. Add `.cargo/config.toml` to your workspace:
   
   ```toml
   [target.wasm32-wasip1]
   runner = 'wasmedge --dir .:.'
   rustflags = ["--cfg", "wasmedge", "--cfg", "tokio_unstable"]
   
   [target.wasm32-unknown-unknown]
   runner = 'wasm-bindgen-test-runner'
   ```

5. Add dev dependency to the `Cargo.toml`:
   
   ```toml
   [dev-dependencies]
   insta = { version = "1.43.0", git = "https://github.com/druide/insta.git", features = ["yaml"] }
   ```

6. Run tests with the `WasmEdge`:
   
   ```bash
   cargo test --target wasm32-wasip1 -- --nocapture
   
   # or if you have a workspace
   cargo test --target wasm32-wasip1 -p your-lib-name --lib -- --nocapture
   ```

7. Run tests in the browser:
   
   ```bash
   wasm-pack test --chrome
   
   # or directly with the cargo test
   NO_HEADLESS=1 WASM_BINDGEN_USE_BROWSER=1 cargo test --target wasm32-unknown-unknown -- --nocapture
   
   # Windows version
   set NO_HEADLESS=1 && set WASM_BINDGEN_USE_BROWSER=1 && cargo test --target wasm32-unknown-unknown -- --nocapture
   ```
   
   To make file-based asserts working in the browser, add the file `test.rs` to your lib:
   
   ```rust
   /// Initialize test FS.
   #[cfg(test)]
   pub fn init() {
       // should be in a standalone file to avoid duplicates
       insta::include_dir!("tests/snapshots");
   }
   ```
   
   and this before tests:
   
   ```rust
   #[cfg_attr(all(target_family = "wasm", target_os = "unknown"), wasm_bindgen_test)]
   #[test]
   fn test_1() {
       crate::test::init();
   
       // test...
       let v = "some text";
       assert_snapshot!(v);
   }
   ```

8. Default snapshots location is `tests/snapshots`.

## Introduction

Snapshots tests (also sometimes called approval tests) are tests that
assert values against a reference value (the snapshot). This is similar
to how `assert_eq!` lets you compare a value against a reference value but
unlike simple string assertions, snapshot tests let you test against complex
values and come with comprehensive tools to review changes.

Snapshot tests are particularly useful if your reference values are very
large or change often.

## Example

```rust
#[test]
fn test_hello_world() {
    insta::assert_debug_snapshot!(vec![1, 2, 3]);
}
```

Curious? There is a screencast that shows the entire workflow: [watch the insta
introduction screencast](https://www.youtube.com/watch?v=rCHrMqE4JOY&feature=youtu.be).
Or if you're not into videos, read the [5 minute introduction](https://insta.rs/docs/quickstart/).

Insta also supports inline snapshots which are stored right in your source file
instead of separate files. This is accomplished by the companion
[cargo-insta](https://github.com/mitsuhiko/insta/tree/master/cargo-insta) tool.

## Editor Support

For looking at `.snap` files there is a [vscode extension](https://github.com/mitsuhiko/insta/tree/master/vscode-insta)
which can syntax highlight snapshot files, review snapshots and more. It can be installed from the
marketplace: [view on marketplace](https://marketplace.visualstudio.com/items?itemName=mitsuhiko.insta).

![jump to definition](https://raw.githubusercontent.com/mitsuhiko/insta/master/vscode-insta/images/jump-to-definition.gif)

## Diffing

Insta uses [`similar`](https://github.com/mitsuhiko/similar) for all its diffing
operations. You can use it independently of insta. You can use the
[`similar-asserts`](https://github.com/mitsuhiko/similar-asserts) crate to get
inline diffs for the standard `assert_eq!` macro to achieve insta like diffs
for regular comparisons:

```rust
use similar_asserts::assert_eq;

fn main() {
    let reference = vec![1, 2, 3, 4];
    assert_eq!(reference, (0..4).collect::<Vec<_>>());
}
```

## Sponsor

If you like the project and find it useful you can [become a
sponsor](https://github.com/sponsors/mitsuhiko).

## License and Links

- [Project Website](https://insta.rs/)
- [Documentation](https://docs.rs/insta/)
- [Issue Tracker](https://github.com/mitsuhiko/insta/issues)
- License: [Apache-2.0](https://github.com/mitsuhiko/insta/blob/master/LICENSE)
