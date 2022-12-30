# Cairo 1 template

## How to use

* Install [Rust](https://www.rust-lang.org/tools/install)
* Click the "Use this template" button to use this as a basis for your repo or create a codespace on Github.
* To build and run your cairo programs use `cargo run --bin cairo-run -- -p /path/to/file.cairo` For example in the template you would use `cargo run --bin cairo-run -- -p src/main.cairo`

## Testing

* Add unit tests in `tests/test.rs` file. There are already tests for example programs.
* Run with `cargo test --test test`

## Examples

### `power.cairo`

Introduces:
* recursion and conditional statements

### `safe_division.cairo`

Introduces:
* `u128` unsigned integer 128 data type
* safe integer division with `felt` and `u128` data types
* testing for code failing to execute. This is represented by `panic!` macro executed by Cairo's VM Rust implementation.
