## Technicalities

1. `src/lib.rs`

   + The `run` function is public and async because starts an HTTP server and is not an entrypoint to the application.

   + This function invokes `HTTPServer::run` and returns an instance of `Server`. It handles incoming requests as they arrive and the shutdowns (or completes).

   + There is no `await` because this function would then listen on the address indefinitely, without completing.

2. `src/main.rs`

   + Its returns an `io::Error` if the provided address fails to bind. If there is no issue with address binding, it calls `await` on the server.

3. `tests/greet.rs`

   + The `spawn_app` function performs setup to launch the server as a background task and panics if this setup cant be done.

   + It then returns a handle to the spawned future (indicated by `_` in the non-binding `let`).

   + The `greeting_works` function calls the `spawn_app` function, creates a request client, and captures a response by sending a `GET` request to the endpoint.

   + This function is `async` because it calls `await`. It tests for the response status, content length, header type, and response body.

   + The test for response body is at the end because the `text` method consumes the response. This means that the response object can no longer be used after this call.