[![MailRocket](https://github.com/saurabhmshr/mailrocket/actions/workflows/ci.yaml/badge.svg)](https://github.com/saurabhmshr/mailrocket/actions/workflows/ci.yaml)

# Mail Rocket

Email newsletter backend based on [Zero to Production in Rust][1] by [Luca Palmieri's][2]

## Documentation

Each release has documentation about features and improvements. This release has documents for:

+ [Application Anatomy][3]

+ [Project Structure][4]

+ [Testing][5]

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

+ Verify application (**in a separate tab or window**)

  ```
  curl 127.0.0.1:8080/hello
  ```

[1]: https://www.zero2prod.com/
[2]: https://www.lpalmieri.com/
[3]: ./docs/app-anatomy.md
[4]: ./docs/directory-structure.md
[5]: ./docs/testing.md