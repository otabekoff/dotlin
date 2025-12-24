# Changelog

All notable changes to the Dotlin programming language will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- File I/O runtime functions (`dotlin_file_read`, `dotlin_file_write`, `dotlin_file_append`, `dotlin_file_exists`)
- Enhanced console I/O (`dotlin_print`, `dotlin_read_line`)
- Standard library foundation (`dotlin_stdlib` crate)
- Cross-platform installation scripts (Unix and Windows)
- GitHub Actions CI/CD workflow
- Comprehensive documentation (README, CONTRIBUTING, ROADMAP, STATUS, PROGRESS)

### Changed
- Organized project structure with proper directories for docs, examples, and tests

## [0.1.0-alpha] - 2024-12-24

### Added
- **Core Language Features**
  - Lexer with support for keywords, identifiers, literals, and operators
  - Recursive descent parser with expression precedence
  - Type system with Int, Float, String, and Boolean types
  - Type inference and type checking pass
  - AST type annotation
  
- **Code Generation**
  - Cranelift IR backend for native compilation
  - Type-aware code generation
  - Float arithmetic and comparisons
  - String operations (concatenation, comparison)
  - Member access support (`.` operator)
  - Optimization enabled (speed level)

- **Runtime Library**
  - `println_i64` - Print 64-bit integers
  - `println_f64` - Print 64-bit floats
  - `println_str` - Print length-prefixed strings
  - `dotlin_string_concat` - String concatenation
  - `dotlin_string_compare` - Lexicographical string comparison

- **Interpreter**
  - Tree-walk interpreter for REPL
  - Full support for all language features
  - Member access evaluation

- **Tooling**
  - `dotc` - Compiler CLI with linking support
  - `dotrepl` - Interactive REPL
  - VS Code syntax highlighting extension

- **Type System Features**
  - Static type checking
  - Type inference for variables
  - Member access validation
  - Function type checking

- **String Features**
  - String concatenation with `+` operator
  - String comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`)
  - `.length` property for strings
  - Length-prefixed string representation

- **Float Features**
  - Float literal support
  - Float arithmetic (`+`, `-`, `*`, `/`)
  - Float comparisons
  - Float negation

### Changed
- Refactored Expression AST to include type annotations
- Updated parser to support postfix expressions
- Enhanced codegen to be type-aware

### Fixed
- Misaligned pointer dereference in string concatenation
- Type mismatches in codegen
- Unused variable warnings

## [0.0.1] - 2024-10-01

### Added
- Initial project setup
- Basic lexer and parser
- Simple AST structure
- Proof-of-concept code generation

---

## Version History

- **v0.1.0-alpha** (2024-12-24): Core language implementation complete
- **v0.2.0** (Planned Feb 2025): Standard library with arrays, file I/O, math
- **v0.3.0** (Planned Mar 2025): Package manager
- **v0.4.0** (Planned Apr 2025): Enhanced tooling (LSP, formatter, linter)
- **v0.5.0** (Planned Jun 2025): Advanced features (generics, traits, closures)
- **v1.0.0** (Planned Oct 2025): Public release

[Unreleased]: https://github.com/dotlin-lang/dotlin/compare/v0.1.0-alpha...HEAD
[0.1.0-alpha]: https://github.com/dotlin-lang/dotlin/releases/tag/v0.1.0-alpha
[0.0.1]: https://github.com/dotlin-lang/dotlin/releases/tag/v0.0.1
