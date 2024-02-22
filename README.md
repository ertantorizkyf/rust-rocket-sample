# rust-rocket-sample

## Description
This is a sample rust backend project implementing rocket framework

## Project structure
- src/main.rs - Mount all APIs and declare modules
- assets/* - Contain sample files for fs implementation
- src/handlers/* - API logic handler
- src/helpers/* - Set of functions to perform specific function, called by handlers
- src/models/* - Data structure
- src/responses/* - Response structure

## Running the app
- debug mode: `cargo watch -q -c -w src/ -x run`
- release mode: `cargo run --release`
  
