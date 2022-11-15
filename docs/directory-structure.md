## Project Structure

+ The project is refactored into a binary and library. The former will server as an entrypoint into the application (and will have minimal code), and the latter will contain logic for the application.

+ Manifest file `Cargo.toml` has a [lib] section and an explicitly defined [[bin]] section containting paths to the library and binary respectively.