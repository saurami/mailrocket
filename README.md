[![MailRocket](https://github.com/saurabhmshr/mailrocket/actions/workflows/ci.yaml/badge.svg)](https://github.com/saurabhmshr/mailrocket/actions/workflows/ci.yaml)

# mailrocket

Email newsletter backend

## Application Anatomy

+ Server: `HTTPServer`

  HTTP Server handles transport level by listening to incoming requests (on a TCP socket).

+ Application: `App`

  App is a builder pattern which contains application logic - routing, middleware, request handlers, etc. to handle client requests and send a response.

+ Endpoint: `Route`

  Route combines a handler (path) with a set of guards (conditions that a request must satisfy for matching)

+ Handler: `greet()`

  Implements `Responder` because the string type can be converted into a HTTPResponse.

+ Asynchronous `main`

  Asynchronous programming is built on top of the `Future` trait - a value that may not exist. All futures expose a `poll` method which has to be called so that the future can resolve to its final value. Rust futures are lazy - unless polled, there is no guarantee it will execute to completion.

  Rust's standard library does not include asynchronous runtime, and it needs to be added as a dependency. The asynchronous runtime on top of `main` drives the futures to completion. Therefore, `main` is asynchronous because `HTTPServer::run` is an asynchronous method.

## Flow

The application iterates over all registered endpoints until it finds a match, and then passes the request object to the handler.

## Getting Started

+ Add and install project dependencies

  ```
  cargo add actix-web
  cargo add tokio --features "macros rt-multi-thread"
  ```

+ Compile project (and check dependencies for errors)

  ```
  cargo check
  ```

+ Run integration test

  ```
  cargo test
  ```

+ Run web server

  ```
  cargo run
  ```

+ Verify application (**in a separate tab**)

  ```
  curl 127.0.0.1:8080/hello
  ```
