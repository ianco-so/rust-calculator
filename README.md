# Introduction

This programming project provides a simple command-line calculator that allows users to write and evaluate math expressions using the `meval` crate in Rust.

# Organization

The folders and files of this project are the following:

* `src/main.rs`: This file contains the main implementation of the calculator. It reads user input, processes the expressions, and evaluates them using `meval`.
* `Cargo.toml`: This file specifies the dependencies and metadata for the Rust project.
* `README.md`: This file.

# Running the Calculator

To run this project, you need to have [Rust](https://www.rust-lang.org/) installed. Follow these steps to run the calculator:

1. Clone the repository:
   ```sh
   git clone https://github.com/ianco-so/rust-calculator.git
   cd rust-calculator
   ```
2. Run the project using Cargo:
    Build the project using Cargo:
    ```sh
    cargo build
    ```
    Run the project using Cargo:
    ```sh
    cargo run
    ```
    Alternatively, you can build and run the project without Cargo:
    Build the project:
    ```sh
    rustc src/main.rs
    ```
    Run the project:
    ```sh
    ./main # or main.exe on Windows
    ```
# Usage

The calculator supports basic arithmetic operations and functions. Here are some examples of valid expressions examples:
```sh
$ x = 10
result: 10
$ x + 2
result: 12
$ x / b
error: Evaluation error
$ b = x + 12
result: 22
$ x = b / x
result: 2.2
$ x
result: 2.2
```

# Dependencies

The project uses the following Crates dependencies:

[**meval:**](https://crates.io/crates/meval) A crate for parsing and evaluating mathematical expressions in Rust. It is specified in the Cargo.toml file.
You can add more dependencies as needed by updating the Cargo.toml file.

# Authorship

Program developed by Ianco (<ianco.so@gmail.com>), 2024.1

&copy; IMD/UFRN.