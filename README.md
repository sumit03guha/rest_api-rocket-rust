# A very basic web server built on top of the Rocket web framework using Rust

This web server is a very basic implementation of a REST API server built on top of the [Rocket](https://rocket.rs/) web framework using [Rust](https://www.rust-lang.org/), where the user can **add**, **delete** and **get** items from a `To-Do list`.

## Useful links

- [Installing rust](https://github.com/abhi3700/My_Learning-Rust#installation)
- [Rocket](https://rocket.rs/)
- [rusqlite](https://docs.rs/rusqlite/latest/rusqlite/)
- [Rust](https://www.rust-lang.org/)

## Useful toolchain extensions

- Recommended extension to install that watching and auto-reloading of the application through the use of `cargo watch`:

  ```shell
  cargo install cargo-watch
  ```

## Useful VSCode extensions

- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Compiling and Starting the Server

- To compile for development:

  ```shell
  cargo build
  ```

  This creates a binary in the `target/debug/rest_api-rocket-rust` directory.

- If `cargo-watch` is installed, then the following command can be used to compile and start the server:

  ```shell
  cargo watch -x run
  ```

  which will create the corresponding binary in target/release:

  ```shell
  target/release/rest_api-rocket-rust
  ```

  This will create a file `data.sqlite` in the root directory of the project.\
  The server will, by default, start an HTTP API server on port `8000` and will only be accessible as localhost.

## Sending requests to the server

- To send a request to the server, use the following command in your terminal after starting the server:

  ```shell
  curl localhost:8000
  ```

  This will return the following response:

  ```json
  {
    "message": "Hello, world!"
  }
  ```

  ```shell
  curl localhost:8000/todo
  ```

  This will return the following response:

  ```json
  {
    "items": []
  }
  ```

  This will empty for the first time, since we have not added any items to the list yet.

- To add an item to the list, use the following command:

  ```shell
  curl localhost:8000/todo -X POST -d '"Item number 1"' -H "Content-Type: application/json"
  ```

  This will return the following response:

  ```json
  {
    "message": "1 rows inserted"
  }
  ```

- To get the list of items, use the following command:

  ```shell
  curl localhost:8000/todo
  ```

  This will return the following response:

  ```json
  {
    "items": [
      {
        "id": 1,
        "item": "Item number 1"
      }
    ]
  }
  ```

- To delete an item from the list, use the following command by specifying the id of the item:

  ```shell
  curl localhost:8000/todo/1 -X DELETE
  ```

  This will return the following response:

  ```json
  {
    "message": "1 rows deleted"
  }
  ```
