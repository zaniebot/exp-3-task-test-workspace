# Contributing to sandbox

Thank you for your interest in contributing to this project! This guide will help you get started.

## Setting Up the Development Environment

1. **Install Rust**: Install the Rust toolchain using [rustup](https://rustup.rs/):

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Verify the installation**:

   ```sh
   rustc --version
   cargo --version
   ```

3. **Clone the repository**:

   ```sh
   git clone https://github.com/zaniebot/exp-3-task-test-workspace.git
   cd exp-3-task-test-workspace
   ```

## Building and Testing

- **Build the project**:

  ```sh
  cargo build
  ```

- **Run the tests**:

  ```sh
  cargo test
  ```

- **Run the project**:

  ```sh
  cargo run
  ```

## Code Style Expectations

Before submitting any changes, please ensure your code meets the project's quality standards:

1. **Format your code** using `cargo fmt`:

   ```sh
   cargo fmt
   ```

2. **Run the linter** using `cargo clippy` and address any warnings:

   ```sh
   cargo clippy -- -D warnings
   ```

3. **Ensure all tests pass**:

   ```sh
   cargo test
   ```

## Submitting Pull Requests

1. **Fork the repository** and create a new branch from `main` for your changes:

   ```sh
   git checkout -b my-feature
   ```

2. **Make your changes** in small, focused commits with clear commit messages.

3. **Run the full check suite** before pushing:

   ```sh
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```

4. **Push your branch** and open a pull request against the `main` branch.

5. **In your pull request description**, please include:
   - A clear summary of what the change does
   - The motivation or context for the change
   - Any relevant issue numbers (e.g., "Closes #123")

6. **Be responsive to feedback** — maintainers may request changes before merging.

## Reporting Issues

If you find a bug or have a feature request, please open an issue on the GitHub repository with a clear description and, if applicable, steps to reproduce the problem.
