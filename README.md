# Dotlin Programming Language

<div align="center">

**A modern, type-safe programming language with native compilation**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/dotlin-lang/dotlin)
[![Version](https://img.shields.io/badge/version-0.1.0--alpha-blue)](https://github.com/dotlin-lang/dotlin/releases)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-orange)](LICENSE)

[Getting Started](#getting-started) • [Documentation](docs/) • [Examples](examples/) • [Roadmap](docs/ROADMAP_V1.md)

</div>

## Overview

Dotlin is a statically-typed programming language that combines the safety of strong type systems with the performance of native compilation. Built on Cranelift, Dotlin compiles to efficient machine code while providing modern language features.

### Key Features

- ✅ **Strong Type System**: Static typing with type inference
- ✅ **Native Compilation**: Compiles to native code via Cranelift
- ✅ **Modern Syntax**: Clean, readable syntax inspired by Kotlin and Rust
- ✅ **Type Safety**: Catch errors at compile time, not runtime
- ✅ **Fast Compilation**: Quick compile times for rapid development
- ✅ **Interactive REPL**: Experiment and prototype quickly
- 🚧 **Standard Library**: Growing collection of essential utilities
- 🚧 **Package Manager**: Easy dependency management (coming soon)

## Quick Start

### Hello, World!

```dotlin
fun main() {
    println("Hello, World!")
}
```

### More Examples

```dotlin
// Type-safe variables
var name: String = "Dotlin"
var version: Float = 0.1
var isAwesome: Boolean = true

// Float arithmetic
var pi = 3.14159
var area = pi * 5.0 * 5.0
println(area)  // 78.53975

// String operations
var greeting = "Hello, " + "World!"
println(greeting.length)  // 13

// String comparison
if ("apple" < "banana") {
    println("Lexicographical ordering!")
}

// Functions
fun greet(name: String): String {
    return "Hello, " + name + "!"
}

println(greet("Dotlin"))
```

## Installation

### From Source (Current)

```bash
# Clone the repository
git clone https://github.com/dotlin-lang/dotlin
cd dotlin

# Build with Cargo
cargo build --release

# Binaries will be in target/release/
# - dotc: Compiler
# - dotrepl: Interactive REPL
```

### One-Line Install (Coming Soon)

```bash
# Unix/Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://dotlin.dev/install.sh | sh

# Windows
iwr -useb https://dotlin.dev/install.ps1 | iex
```

### Package Managers (Planned)

```bash
brew install dotlin          # macOS
choco install dotlin         # Windows
sudo apt install dotlin      # Debian/Ubuntu
cargo install dotlin         # Rust users
```

## Usage

### Compile and Run

```bash
# Compile a Dotlin file
dotc hello.lin -o hello

# Run the executable
./hello  # Unix
hello.exe  # Windows
```

### Interactive REPL

```bash
dotrepl
```

```dotlin
>> var x = 42
>> var y = x * 2
>> println(y)
84
```

## Language Features

### Type System

Dotlin supports the following types:

- **Int**: 64-bit signed integers
- **Float**: 64-bit floating-point numbers
- **String**: UTF-8 encoded strings
- **Boolean**: true/false values

```dotlin
var count: Int = 42
var pi: Float = 3.14159
var name: String = "Dotlin"
var flag: Boolean = true
```

### Control Flow

```dotlin
// If expressions
if (x > 0) {
    println("Positive")
} else {
    println("Non-positive")
}

// While loops
var i = 0
while (i < 10) {
    println(i)
    i = i + 1
}
```

### Functions

```dotlin
fun add(a: Int, b: Int): Int {
    return a + b
}

fun greet(name: String): String {
    return "Hello, " + name
}
```

### String Operations

```dotlin
var s1 = "Hello"
var s2 = "World"

// Concatenation
var greeting = s1 + ", " + s2 + "!"

// Length property
var len = greeting.length

// Comparison
if (s1 < s2) {
    println("s1 comes first")
}
```

## Project Structure

```
dotlin/
├── crates/              # Rust crates
│   ├── dotlin_ast/      # Abstract Syntax Tree
│   ├── dotlin_lexer/    # Tokenization
│   ├── dotlin_parser/   # Parser
│   ├── dotlin_typechecker/  # Type checking
│   ├── dotlin_codegen/  # Code generation (Cranelift)
│   ├── dotlin_runtime/  # Runtime library
│   ├── dotlin_interpreter/  # REPL interpreter
│   ├── dotlin_stdlib/   # Standard library (in progress)
│   ├── dotc/            # Compiler CLI
│   └── dotrepl/         # REPL CLI
├── docs/                # Documentation
│   ├── language-guide/  # Language reference
│   ├── api/             # API documentation
│   └── tutorials/       # Tutorials and guides
├── examples/            # Example programs
│   ├── basic/           # Basic examples
│   ├── intermediate/    # Intermediate examples
│   └── advanced/        # Advanced examples
├── tests/               # Test suite
│   ├── integration/     # Integration tests
│   └── unit/            # Unit tests
├── scripts/             # Build and installation scripts
└── editors/             # Editor support
    └── vscode/          # VS Code extension
```

## Development Status

### Current Version: v0.1.0-alpha

**Completed:**
- ✅ Lexer and Parser
- ✅ Type System with Inference
- ✅ Type Checker
- ✅ Cranelift Code Generation
- ✅ Runtime Library
- ✅ REPL Interpreter
- ✅ Basic String Operations
- ✅ Float Support
- ✅ VS Code Syntax Highlighting

**In Progress (v0.2.0):**
- 🚧 Standard Library
  - File I/O operations
  - Array/Vector type
  - Math module
  - Enhanced console I/O

**Planned:**
- 📋 Package Manager (`dotpkg`)
- 📋 Language Server Protocol (LSP)
- 📋 Code Formatter
- 📋 Linter
- 📋 Generics
- 📋 Pattern Matching
- 📋 Error Handling (Result/Option types)

See [ROADMAP](docs/ROADMAP_V1.md) for detailed plans.

## Roadmap to v1.0

| Version | Features | Target | Status |
|---------|----------|--------|--------|
| v0.1.0 | Core Language | Dec 2024 | ✅ Complete |
| v0.2.0 | Standard Library | Feb 2025 | 🚧 In Progress |
| v0.3.0 | Package Manager | Mar 2025 | 📋 Planned |
| v0.4.0 | Enhanced Tooling | Apr 2025 | 📋 Planned |
| v0.5.0 | Advanced Features | Jun 2025 | 📋 Planned |
| v0.6.0 | Cross-Platform Build | Jul 2025 | 📋 Planned |
| v0.7.0 | Installation System | Jul 2025 | 📋 Planned |
| v0.8.0 | Documentation | Aug 2025 | 📋 Planned |
| v0.9.0 | Testing & QA | Sep 2025 | 📋 Planned |
| **v1.0.0** | **Public Release** | **Oct 2025** | 📋 **Planned** |

## Documentation

- [Language Guide](docs/language-guide/) - Complete language reference
- [API Documentation](docs/api/) - Standard library API
- [Tutorials](docs/tutorials/) - Step-by-step guides
- [Roadmap](docs/ROADMAP_V1.md) - Development roadmap
- [Status](docs/STATUS.md) - Current implementation status
- [Progress](docs/PROGRESS.md) - Development progress

## Examples

Check out the [examples](examples/) directory for sample programs:

- [Basic Examples](examples/basic/) - Hello World, variables, functions
- [Intermediate Examples](examples/intermediate/) - File I/O, data structures
- [Advanced Examples](examples/advanced/) - Complex applications

## Contributing

Dotlin is in active development and contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/dotlin-lang/dotlin
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

## Community

- **GitHub**: [github.com/dotlin-lang/dotlin](https://github.com/dotlin-lang/dotlin)
- **Website**: [dotlin.dev](https://dotlin.dev) (coming soon)
- **Discord**: (coming soon)
- **Documentation**: [dotlin.dev/docs](https://dotlin.dev/docs) (coming soon)

## License

Dotlin is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## Acknowledgments

Dotlin is built with:

- [Cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift) - Code generation backend
- [Logos](https://github.com/maciejhirsz/logos) - Lexer generator
- [Rust](https://www.rust-lang.org/) - Implementation language

## Authors

- **Marko** - Initial work and development

See also the list of [contributors](https://github.com/dotlin-lang/dotlin/contributors) who participated in this project.

---

<div align="center">

**Made with ❤️ by the Dotlin Team**

[⬆ Back to Top](#dotlin-programming-language)

</div>
