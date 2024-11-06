# async_refactor

A library for automated async code refactoring in Rust.

## Features

- Extract async functions from code snippets.
- Convert synchronous functions to async.

## Installation

To use `async_refactor`, add the following to your `Cargo.toml`:

```toml
[dependencies]
async_refactor = "0.1.0"
```
## Usage
```rust
use async_refactor::async_refactor;

fn main() {
    let code = "let x = 5;";
    
    let extracted_async = async_refactor::extract_async_function(code, "my_async_func");
    println!("{}", extracted_async);
}
```
## License
This project is licensed under the MIT License

## Running the Project

1. **Build the project**:
   ```sh
   cargo build
   ```
Run the tests:
```sh
cargo test
```
## Author
Ben Santora <bensatlantik@gmail.com>

## Donate
https://bensatlantik.github.io/

