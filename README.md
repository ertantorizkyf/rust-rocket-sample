# rust-rocket-sample

## Description
This is a sample rust backend project implementing rocket framework

## Project structure
- src/main.rs - Mount all APIs and declare modules
- assets/* - Contain sample files for fs implementation
- assets/Rust Rocket Sample.postman_collection.json - Contain list of available APIs along with its request query or body
- src/handlers/* - API logic handler
- src/helpers/* - Set of functions to perform specific function, called by handlers
- src/models/* - Data structure
- src/responses/* - Response structure
- src/constants/* - Store constant values
- src/fairings/* - Middleware implementation

## Running the app
- debug mode: `cargo watch -q -c -w src/ -x run`
- release mode: `cargo run --release`
  
