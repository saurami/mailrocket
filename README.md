[![MailRocket](https://github.com/saurabhmshr/mailrocket/actions/workflows/ci.yaml/badge.svg)](https://github.com/saurabhmshr/mailrocket/actions/workflows/ci.yaml)

# Mail Rocket

Email newsletter backend based on [Zero to Production in Rust][1] by [Luca Palmieri][2]

## Documentation

Each release has documentation about features and improvements. This release has documents for:

+ [Application Anatomy][3]

+ [Project Structure][4]

+ [Testing][5]

+ [Technicalities][6]

## Getting Started

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

  **Note:** Export these environment variables only if `cargo run` doesn't show the bind address and port

  Powershell: `$env:RUST_LOG="trace"`
  Bash / Zsh: `export RUST_LOG="trace"`

+ Verify application (**in a separate tab or window**)

  ```
  curl 127.0.0.1:PORT/hello
  curl 127.0.0.1:PORT/health-check
  ```

[1]: https://www.zero2prod.com/
[2]: https://www.lpalmieri.com/
[3]: ./docs/app-anatomy.md
[4]: ./docs/directory-structure.md
[5]: ./docs/testing.md
[6]: ./docs/technicalities.md
