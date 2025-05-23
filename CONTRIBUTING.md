# Contributing to Angelax

First off, thank you for considering contributing to Angelax! I'm excited to build the objectively best backend framework with the help of the community. Your contributions are valuable to me.

This document provides guidelines for contributing to Angelax. Please read it to ensure a smooth and effective contribution process.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Your First Code Contribution](#your-first-code-contribution)
  - [Pull Requests](#pull-requests)
- [Style Guides](#style-guides)
  - [Git Commit Messages](#git-commit-messages)
  - [Rust Style Guide](#rust-style-guide)
  - [Documentation Style Guide](#documentation-style-guide)
- [Development Setup](#development-setup)
- [Testing](#testing)
- [Community and Communication](#community-and-communication)

## Code of Conduct

This project and everyone participating in it is governed by the [Angelax Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to [carterperez@certgames.com].

## How Can I Contribute?

### Reporting Bugs

If you find a bug, please ensure the bug was not already reported by searching on GitHub under [Issues]([https://github.com/CarterPerez-dev/Angela/issues]).

If you're unable to find an open issue addressing the problem, [open a new one]([https://github.com/CarterPerez-dev/Angela/issues/new]). Be sure to include a **title and clear description**, as much relevant information as possible, and a **code sample or an executable test case** demonstrating the expected behavior that is not occurring. Use the "Bug Report" issue template.


Working on your first Pull Request? Here are a couple of friendly tutorials:
- [How to Contribute to an Open Source Project on GitHub](https://egghead.io/courses/how-to-contribute-to-an-open-source-project-on-github)
- [First Contributions](https://github.com/firstcontributions/first-contributions)

### Pull Requests

1.  **Fork the repository** and create your branch from `main` (or the current development branch).
2.  **Set up your development environment** (see [Development Setup](#development-setup)).
3.  **Make your changes.** Add tests for new features or bug fixes.
4.  **Ensure the test suite passes** (`cargo test --all-crates --all-features`).
5.  **Format your code** (`cargo fmt --all`).
6.  **Lint your code** (`cargo clippy --all-targets --all-features -- -D warnings`).
7.  **Create a pull request** to the `main` branch (or development branch).
8.  Clearly describe your changes in the PR description. Link to any relevant issues.
9.  Ensure your PR follows the [Git Commit Messages](#git-commit-messages) style.

## Style Guides

### Git Commit Messages

- Use the present tense ("Add feature" not "Added feature").
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...").
- Limit the first line to 72 characters or less.
- Reference issues and pull requests liberally after the first line.
- Consider using [Conventional Commits](https://www.conventionalcommits.org/) for structured messages (e.g., `feat:`, `fix:`, `docs:`, `chore:`).

Example:
```
feat(core): Add support for HTTP/3 request parsing

Implements the initial parser for HTTP/3 requests based on the QUIC
transport layer. This is a foundational step towards full HTTP/3 support.

Fixes #123
Related to #456
```

### Rust Style Guide

- Follow the standard [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) and `rustfmt` conventions.
- Run `cargo fmt --all` before committing.
- Run `cargo clippy --all-targets --all-features -- -D warnings` to catch common mistakes and ensure code quality. Address all Clippy warnings.

### Documentation Style Guide

- Write clear, concise, and helpful documentation.
- Use Markdown for documentation files.
- For Rust code, write comprehensive doc comments (`///` for items, `//!` for modules/crates).
- Follow Rustdoc conventions.

## Development Setup

To get Angelax up and running for development:

1.  **Install Rust:** Follow the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).
2.  **Clone the repository:**
    ```bash
    git clone https://github.com/CarterPerez/Angelax.git
    cd Angelax
    ```
3.  **Build the project:**
    ```bash
    cargo build --all-crates
    ```
4.  **Run tests:**
    ```bash
    cargo test --all-crates
    ```
*(Add any other specific setup steps, like installing build dependencies, setting up hooks, etc.)*

## Testing

- All new features and bug fixes **must** include tests.
- Run `cargo test --all-crates --all-features` to execute all tests.
- We aim for high test coverage. Consider adding unit, integration, and (where appropriate) benchmarks.

## Community and Communication

- **GitHub Issues:** For bugs and feature requests.
- **https://discord.com/certgames:** 

We look forward to your contributions!
