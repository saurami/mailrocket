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
