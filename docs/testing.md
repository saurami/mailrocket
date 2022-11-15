## Testing

Two kinds of tests are commonly used in Rust projects:

1. Embedded Tests

  + These tests have privileged to the program - they can interact with structs, methods, fields, etc. This is called white-box testing as they have access to the underlying code.

  + They are part of the project and make the application binary bigger

2. External Tests

  + These are integration tests and are used to test code like an end user would. This is called black box testing - verifying behavior of a system by examining its output, given a set of input, without having access access to the internal implementation.

  + They are present is the `./tests` directory and are imported as a crate. They are also not part of the application binary.
