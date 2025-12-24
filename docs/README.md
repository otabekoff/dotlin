# Dotlin Documentation

Welcome to the Dotlin programming language documentation!

## Documentation Structure

### [Language Guide](language-guide/)
Complete reference for the Dotlin programming language:
- Getting Started
- Syntax and Semantics
- Type System
- Standard Library
- Best Practices

### [API Documentation](api/)
Detailed API documentation for:
- Standard Library
- Runtime Functions
- Compiler API
- REPL API

### [Tutorials](tutorials/)
Step-by-step guides and tutorials:
- Your First Dotlin Program
- Working with Types
- File I/O
- Building Applications

## Quick Links

- [README](../README.md) - Project overview
- [ROADMAP](ROADMAP_V1.md) - Development roadmap to v1.0
- [STATUS](STATUS.md) - Current implementation status
- [PROGRESS](PROGRESS.md) - Development progress tracking
- [CHANGELOG](../CHANGELOG.md) - Version history
- [CONTRIBUTING](../CONTRIBUTING.md) - How to contribute

## Getting Started

### Installation

See the [main README](../README.md#installation) for installation instructions.

### Your First Program

Create a file `hello.lin`:

```dotlin
fun main() {
    println("Hello, Dotlin!")
}
```

Compile and run:

```bash
dotc hello.lin -o hello
./hello
```

### Language Basics

#### Variables

```dotlin
var name: String = "Dotlin"
var count: Int = 42
var pi: Float = 3.14159
var flag: Boolean = true
```

#### Functions

```dotlin
fun add(a: Int, b: Int): Int {
    return a + b
}

fun greet(name: String): String {
    return "Hello, " + name
}
```

#### Control Flow

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

## Language Features

### Current Features (v0.1.0-alpha)

- ✅ Static typing with type inference
- ✅ Int, Float, String, Boolean types
- ✅ Functions with parameters and return types
- ✅ If/else expressions
- ✅ While loops
- ✅ String concatenation and comparison
- ✅ String `.length` property
- ✅ Float arithmetic
- ✅ Native compilation via Cranelift
- ✅ Interactive REPL

### Upcoming Features (v0.2.0+)

- 🚧 Arrays and collections
- 🚧 File I/O operations
- 🚧 Math module
- 📋 Result and Option types
- 📋 Pattern matching
- 📋 Generics
- 📋 Traits/Interfaces
- 📋 Closures

## Standard Library

The Dotlin standard library is currently under development. Planned modules include:

- **Collections**: Array, Map, Set
- **I/O**: File operations, console I/O
- **Math**: Mathematical functions and constants
- **String**: String manipulation utilities
- **Error**: Result and Option types

See [stdlib documentation](api/stdlib/) for details.

## Compiler and Tools

### dotc - Compiler

```bash
# Compile a file
dotc input.lin -o output

# Compile only (no linking)
dotc input.lin -c -o output.o

# Specify runtime library path
dotc input.lin -o output --runtime-path /path/to/runtime
```

### dotrepl - Interactive REPL

```bash
# Start the REPL
dotrepl

# In the REPL
>> var x = 42
>> println(x)
42
```

## Development

### Building from Source

```bash
git clone https://github.com/dotlin-lang/dotlin
cd dotlin
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on contributing to Dotlin.

## Community

- **GitHub**: [github.com/dotlin-lang/dotlin](https://github.com/dotlin-lang/dotlin)
- **Website**: [dotlin.dev](https://dotlin.dev) (coming soon)
- **Discord**: (coming soon)

## License

Dotlin is dual-licensed under MIT and Apache 2.0. See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.

---

For more information, see the [main README](../README.md) or visit [dotlin.dev](https://dotlin.dev).
