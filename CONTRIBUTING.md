# Contributing to Dotlin

First off, thank you for considering contributing to Dotlin! It's people like you that make Dotlin such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title**
* **Describe the exact steps to reproduce the problem**
* **Provide specific examples** (code snippets, test files)
* **Describe the behavior you observed** and what you expected
* **Include screenshots** if relevant
* **Specify your environment** (OS, Rust version, Dotlin version)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title**
* **Provide a detailed description** of the suggested enhancement
* **Explain why this enhancement would be useful**
* **List any similar features** in other languages

### Pull Requests

1. Fork the repository
2. Create a new branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run clippy (`cargo clippy`)
6. Format code (`cargo fmt`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## Development Setup

### Prerequisites

* Rust 1.70 or later
* Git
* A C compiler (for linking)

### Building from Source

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/dotlin
cd dotlin

# Build the project
cargo build

# Run tests
cargo test

# Run the compiler
cargo run -p dotc -- examples/basic/hello.lin

# Run the REPL
cargo run -p dotrepl
```

### Project Structure

```
dotlin/
├── crates/              # All Rust crates
│   ├── dotlin_ast/      # AST definitions
│   ├── dotlin_lexer/    # Tokenization
│   ├── dotlin_parser/   # Parser
│   ├── dotlin_typechecker/  # Type checking
│   ├── dotlin_codegen/  # Code generation
│   ├── dotlin_runtime/  # Runtime library
│   ├── dotlin_interpreter/  # REPL interpreter
│   ├── dotlin_stdlib/   # Standard library
│   ├── dotc/            # Compiler CLI
│   └── dotrepl/         # REPL CLI
├── docs/                # Documentation
├── examples/            # Example programs
├── tests/               # Test suite
└── scripts/             # Build scripts
```

## Coding Guidelines

### Rust Code Style

* Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
* Use `cargo fmt` to format code
* Use `cargo clippy` to catch common mistakes
* Write documentation comments for public APIs
* Add tests for new features

### Dotlin Code Style

* Use 4 spaces for indentation
* Use camelCase for variables and functions
* Use PascalCase for types
* Add comments for complex logic
* Keep functions focused and small

### Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters
* Reference issues and pull requests liberally

Example:
```
Add file I/O operations to standard library

- Implement readFile, writeFile, appendFile
- Add runtime functions for file operations
- Include tests and documentation

Fixes #123
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p dotlin_parser

# Run a specific test
cargo test test_name
```

### Writing Tests

* Add unit tests in the same file as the code
* Add integration tests in `tests/` directory
* Test edge cases and error conditions
* Use descriptive test names

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_concatenation() {
        let result = concat("Hello, ", "World!");
        assert_eq!(result, "Hello, World!");
    }
}
```

## Documentation

* Document all public APIs with doc comments
* Include examples in documentation
* Update README.md for user-facing changes
* Add entries to CHANGELOG.md

Example:
```rust
/// Concatenates two strings together.
///
/// # Examples
///
/// ```
/// let result = concat("Hello, ", "World!");
/// assert_eq!(result, "Hello, World!");
/// ```
pub fn concat(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}
```

## Areas for Contribution

### High Priority

* **Standard Library**: Implement arrays, maps, file I/O
* **Error Messages**: Improve compiler error messages
* **Documentation**: Write tutorials and guides
* **Examples**: Create example programs
* **Tests**: Increase test coverage

### Medium Priority

* **Package Manager**: Design and implement `dotpkg`
* **LSP Server**: Language server for editor support
* **Formatter**: Code formatter for Dotlin
* **Linter**: Static analysis tool

### Low Priority

* **Optimization**: Performance improvements
* **Advanced Features**: Generics, traits, pattern matching
* **Tooling**: Additional development tools

## Getting Help

* **GitHub Issues**: For bugs and feature requests
* **Discussions**: For questions and general discussion
* **Discord**: For real-time chat (coming soon)

## Recognition

Contributors will be:

* Listed in the [Contributors](https://github.com/dotlin-lang/dotlin/contributors) page
* Mentioned in release notes for significant contributions
* Given credit in the project documentation

## License

By contributing to Dotlin, you agree that your contributions will be licensed under both the MIT License and Apache License 2.0.

---

Thank you for contributing to Dotlin! 🎉
