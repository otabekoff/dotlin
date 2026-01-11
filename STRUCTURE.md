# Dotlin Project Structure (Rust 2024)

## Complete Directory Layout

```
dotlin/
├── Cargo.toml                      # Workspace root
├── Cargo.lock
├── rust-toolchain.toml             # Rust 2024 edition specification
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                  # Continuous integration
│   │   ├── release.yml             # Release automation
│   │   └── docs.yml                # Documentation deployment
│   └── ISSUE_TEMPLATE/
│       ├── bug_report.md
│       └── feature_request.md
├── .gitignore
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── CHANGELOG.md
├── docs/
│   ├── language-spec/              # Language specification
│   │   ├── 01-syntax.md
│   │   ├── 02-type-system.md
│   │   ├── 03-memory-model.md
│   │   ├── 04-concurrency.md
│   │   └── 05-interop.md
│   ├── compiler/                   # Compiler internals
│   │   ├── architecture.md
│   │   ├── passes.md
│   │   └── optimization.md
│   ├── stdlib/                     # Standard library docs
│   └── tutorials/
│       ├── getting-started.md
│       ├── basic-syntax.md
│       └── advanced-features.md
├── examples/
│   ├── hello_world/
│   │   └── main.lin
│   ├── http_server/
│   │   ├── main.lin
│   │   └── Dotlin.toml
│   ├── concurrent_processing/
│   │   └── pipeline.lin
│   └── ffi_example/
│       ├── main.lin
│       └── native.c
├── benches/
│   ├── compiler_bench.rs           # Compiler performance benchmarks
│   ├── runtime_bench.rs            # Runtime benchmarks
│   └── stdlib_bench.rs             # Standard library benchmarks
├── tests/
│   ├── integration/                # Integration tests
│   │   ├── compile_tests.rs
│   │   ├── runtime_tests.rs
│   │   └── interop_tests.rs
│   └── fixtures/                   # Test fixtures
│       ├── valid/                  # Valid Dotlin programs
│       └── invalid/                # Programs that should fail
│
├── crates/                         # Main workspace crates
│   │
│   ├── dotlin_cli/                 # CLI entry point
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs             # CLI entry point
│   │   │   ├── commands/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── build.rs        # `dotlinc build`
│   │   │   │   ├── run.rs          # `dotlinc run`
│   │   │   │   ├── check.rs        # `dotlinc check`
│   │   │   │   ├── test.rs         # `dotlinc test`
│   │   │   │   ├── fmt.rs          # `dotlinc fmt`
│   │   │   │   ├── doc.rs          # `dotlinc doc`
│   │   │   │   ├── new.rs          # `dotlinc new`
│   │   │   │   └── repl.rs         # `dotlinc repl`
│   │   │   ├── config.rs           # Configuration handling
│   │   │   ├── error.rs            # CLI error handling
│   │   │   └── utils.rs
│   │   └── tests/
│   │       └── cli_tests.rs
│   │
│   ├── dotlin_lexer/               # Lexical analysis
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── token.rs            # Token definitions
│   │   │   ├── lexer.rs            # Main lexer implementation
│   │   │   ├── cursor.rs           # Source code cursor
│   │   │   ├── error.rs            # Lexer errors
│   │   │   └── keywords.rs         # Keyword recognition
│   │   └── tests/
│   │       ├── lexer_tests.rs
│   │       └── token_tests.rs
│   │
│   ├── dotlin_parser/              # Syntax parsing
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── parser.rs           # Main parser
│   │   │   ├── ast/                # Abstract Syntax Tree
│   │   │   │   ├── mod.rs
│   │   │   │   ├── expr.rs         # Expression nodes
│   │   │   │   ├── stmt.rs         # Statement nodes
│   │   │   │   ├── decl.rs         # Declaration nodes
│   │   │   │   ├── types.rs        # Type nodes
│   │   │   │   ├── pattern.rs      # Pattern matching
│   │   │   │   └── visitor.rs      # AST visitor pattern
│   │   │   ├── grammar/            # Grammar rules
│   │   │   │   ├── mod.rs
│   │   │   │   ├── expressions.rs
│   │   │   │   ├── statements.rs
│   │   │   │   ├── declarations.rs
│   │   │   │   └── types.rs
│   │   │   ├── precedence.rs       # Operator precedence
│   │   │   ├── error.rs            # Parse errors
│   │   │   └── span.rs             # Source location tracking
│   │   └── tests/
│   │       ├── parser_tests.rs
│   │       └── ast_tests.rs
│   │
│   ├── dotlin_semantic/            # Semantic analysis
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── analyzer.rs         # Main semantic analyzer
│   │   │   ├── symbol_table.rs     # Symbol tables
│   │   │   ├── scope.rs            # Scope resolution
│   │   │   ├── type_checker/       # Type checking
│   │   │   │   ├── mod.rs
│   │   │   │   ├── inference.rs    # Type inference
│   │   │   │   ├── unification.rs  # Type unification
│   │   │   │   ├── constraints.rs  # Type constraints
│   │   │   │   └── variance.rs     # Variance checking
│   │   │   ├── borrow_checker.rs   # Ownership analysis
│   │   │   ├── null_checker.rs     # Null safety analysis
│   │   │   ├── exhaustiveness.rs   # Pattern exhaustiveness
│   │   │   ├── hir/                # High-level IR
│   │   │   │   ├── mod.rs
│   │   │   │   ├── lowering.rs     # AST -> HIR lowering
│   │   │   │   └── nodes.rs
│   │   │   └── error.rs
│   │   └── tests/
│   │       ├── type_checker_tests.rs
│   │       └── semantic_tests.rs
│   │
│   ├── dotlin_mir/                 # Mid-level IR
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── mir.rs              # MIR definitions
│   │   │   ├── builder.rs          # MIR builder
│   │   │   ├── lowering.rs         # HIR -> MIR lowering
│   │   │   ├── basic_blocks.rs     # Control flow graph
│   │   │   ├── instructions.rs     # MIR instructions
│   │   │   ├── validation.rs       # MIR validation
│   │   │   └── passes/             # MIR transformation passes
│   │   │       ├── mod.rs
│   │   │       ├── simplify.rs
│   │   │       ├── inline.rs
│   │   │       ├── dead_code.rs
│   │   │       └── const_prop.rs
│   │   └── tests/
│   │
│   ├── dotlin_codegen/             # Code generation
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── llvm/               # LLVM backend
│   │   │   │   ├── mod.rs
│   │   │   │   ├── context.rs
│   │   │   │   ├── module.rs
│   │   │   │   ├── types.rs
│   │   │   │   ├── values.rs
│   │   │   │   ├── builder.rs
│   │   │   │   └── optimization.rs
│   │   │   ├── cranelift/          # Cranelift backend (alternative)
│   │   │   │   ├── mod.rs
│   │   │   │   └── generator.rs
│   │   │   ├── native/             # Direct native codegen
│   │   │   │   ├── mod.rs
│   │   │   │   ├── x86_64.rs
│   │   │   │   ├── aarch64.rs
│   │   │   │   └── riscv.rs
│   │   │   ├── object.rs           # Object file generation
│   │   │   ├── linker.rs           # Linking
│   │   │   └── target.rs           # Target specification
│   │   └── tests/
│   │
│   ├── dotlin_runtime/             # Minimal runtime library
│   │   ├── Cargo.toml
│   │   ├── build.rs                # Build script for C components
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── alloc/              # Memory allocation
│   │   │   │   ├── mod.rs
│   │   │   │   ├── arc.rs          # Reference counting
│   │   │   │   ├── arena.rs        # Arena allocator
│   │   │   │   └── bump.rs         # Bump allocator
│   │   │   ├── coroutine/          # Coroutine runtime
│   │   │   │   ├── mod.rs
│   │   │   │   ├── executor.rs
│   │   │   │   ├── scheduler.rs
│   │   │   │   ├── task.rs
│   │   │   │   └── waker.rs
│   │   │   ├── panic.rs            # Panic handling
│   │   │   ├── builtin.rs          # Built-in functions
│   │   │   └── intrinsics.rs       # Compiler intrinsics
│   │   └── c/                      # C components
│   │       ├── runtime.c
│   │       └── runtime.h
│   │
│   ├── dotlin_std/                 # Standard library (written in Dotlin)
│   │   ├── Cargo.toml
│   │   ├── dotlin_sources/         # Dotlin source files
│   │   │   ├── core/
│   │   │   │   ├── prelude.lin
│   │   │   │   ├── primitives.lin
│   │   │   │   ├── option.lin
│   │   │   │   ├── result.lin
│   │   │   │   └── range.lin
│   │   │   ├── collections/
│   │   │   │   ├── list.lin
│   │   │   │   ├── map.lin
│   │   │   │   ├── set.lin
│   │   │   │   └── sequence.lin
│   │   │   ├── io/
│   │   │   │   ├── file.lin
│   │   │   │   ├── stream.lin
│   │   │   │   └── console.lin
│   │   │   ├── concurrent/
│   │   │   │   ├── coroutine.lin
│   │   │   │   ├── channel.lin
│   │   │   │   └── sync.lin
│   │   │   ├── text/
│   │   │   │   ├── string.lin
│   │   │   │   ├── regex.lin
│   │   │   │   └── encoding.lin
│   │   │   ├── net/
│   │   │   │   ├── http.lin
│   │   │   │   ├── socket.lin
│   │   │   │   └── tcp.lin
│   │   │   └── math/
│   │   │       ├── basic.lin
│   │   │       └── complex.lin
│   │   └── src/
│   │       └── lib.rs              # Rust bindings
│   │
│   ├── dotlin_ffi/                 # Foreign Function Interface
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── c_abi.rs            # C ABI support
│   │   │   ├── types.rs            # FFI type conversions
│   │   │   ├── bindings/           # Language bindings
│   │   │   │   ├── mod.rs
│   │   │   │   ├── c.rs
│   │   │   │   ├── cpp.rs
│   │   │   │   ├── rust.rs
│   │   │   │   └── python.rs
│   │   │   └── codegen.rs          # Binding generation
│   │   └── tests/
│   │
│   ├── dotlin_lsp/                 # Language Server Protocol
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs             # LSP server entry
│   │   │   ├── lib.rs
│   │   │   ├── server.rs           # LSP server implementation
│   │   │   ├── handlers/           # LSP request handlers
│   │   │   │   ├── mod.rs
│   │   │   │   ├── completion.rs
│   │   │   │   ├── hover.rs
│   │   │   │   ├── goto_def.rs
│   │   │   │   ├── find_refs.rs
│   │   │   │   ├── rename.rs
│   │   │   │   ├── formatting.rs
│   │   │   │   └── diagnostics.rs
│   │   │   ├── analysis.rs         # Code analysis
│   │   │   └── workspace.rs        # Workspace management
│   │   └── tests/
│   │
│   ├── dotlin_fmt/                 # Code formatter
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── formatter.rs        # Main formatter
│   │   │   ├── rules.rs            # Formatting rules
│   │   │   ├── config.rs           # Formatter configuration
│   │   │   └── visitor.rs          # AST visitor for formatting
│   │   └── tests/
│   │       └── format_tests.rs
│   │
│   ├── dotlin_test/                # Testing framework
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── runner.rs           # Test runner
│   │   │   ├── framework.rs        # Test framework
│   │   │   ├── assertions.rs       # Assertion macros
│   │   │   ├── mocking.rs          # Mocking support
│   │   │   └── reporter.rs         # Test reporting
│   │   └── tests/
│   │
│   ├── dotlin_pkg/                 # Package manager
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── package.rs          # Package metadata
│   │   │   ├── manifest.rs         # Dotlin.toml parsing
│   │   │   ├── resolver.rs         # Dependency resolution
│   │   │   ├── registry/           # Package registry
│   │   │   │   ├── mod.rs
│   │   │   │   ├── client.rs
│   │   │   │   └── cache.rs
│   │   │   ├── download.rs         # Package downloading
│   │   │   ├── lockfile.rs         # Dotlin.lock
│   │   │   └── publish.rs          # Package publishing
│   │   └── tests/
│   │
│   ├── dotlin_diagnostics/         # Error reporting
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── diagnostic.rs       # Diagnostic types
│   │   │   ├── emitter.rs          # Diagnostic emitter
│   │   │   ├── renderer.rs         # Pretty printing
│   │   │   ├── suggestion.rs       # Error suggestions
│   │   │   └── codes.rs            # Error codes
│   │   └── tests/
│   │
│   ├── dotlin_span/                # Source location tracking
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── span.rs             # Span type
│   │   │   ├── source_map.rs       # Source file mapping
│   │   │   └── pos.rs              # Position types
│   │   └── tests/
│   │
│   ├── dotlin_driver/              # Compiler driver
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── driver.rs           # Main driver
│   │   │   ├── session.rs          # Compilation session
│   │   │   ├── pipeline.rs         # Compilation pipeline
│   │   │   ├── queries.rs          # Query system
│   │   │   └── cache.rs            # Incremental compilation
│   │   └── tests/
│   │
│   ├── dotlin_macro/               # Macro system (future)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── expand.rs           # Macro expansion
│   │   │   ├── hygiene.rs          # Hygiene checking
│   │   │   └── builtin.rs          # Built-in macros
│   │   └── tests/
│   │
│   ├── dotlin_symbol/              # Symbol management
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── symbol.rs           # Symbol definitions
│   │   │   ├── interner.rs         # String interning
│   │   │   └── namespace.rs        # Namespaces
│   │   └── tests/
│   │
│   └── dotlin_metadata/            # Metadata serialization
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── encode.rs           # Metadata encoding
│       │   ├── decode.rs           # Metadata decoding
│       │   └── format.rs           # Metadata format
│       └── tests/
│
├── tools/                          # Development tools
│   ├── syntax_gen/                 # Syntax definition generator
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   ├── test_runner/                # Custom test runner
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── perf_analyzer/              # Performance analysis
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
│
├── stdlib_impl/                    # Standard library implementations
│   ├── core/                       # Core library (no_std)
│   │   └── src/
│   │       ├── primitives.rs
│   │       ├── option.rs
│   │       └── result.rs
│   ├── collections/                # Collection implementations
│   │   └── src/
│   │       ├── list.rs
│   │       └── map.rs
│   └── io/                         # I/O implementations
│       └── src/
│           └── file.rs
│
├── scripts/                        # Build and utility scripts
│   ├── build.sh
│   ├── test.sh
│   ├── install.sh
│   ├── bootstrap.sh                # Bootstrap compiler
│   └── release.sh
│
└── ide/                            # IDE integration
    ├── vscode/                     # VS Code extension
    │   ├── package.json
    │   ├── src/
    │   │   ├── extension.ts
    │   │   └── client.ts
    │   └── syntaxes/
    │       └── dotlin.tmLanguage.json
    ├── intellij/                   # IntelliJ IDEA plugin
    │   ├── build.gradle.kts
    │   └── src/
    │       └── main/
    │           └── kotlin/
    └── vim/                        # Vim plugin
        └── syntax/
            └── dotlin.vim
```

---

## Root Configuration Files

### `Cargo.toml` (Workspace Root)

```toml
[workspace]
resolver = "2"
members = [
    "crates/dotlin_cli",
    "crates/dotlin_lexer",
    "crates/dotlin_parser",
    "crates/dotlin_semantic",
    "crates/dotlin_mir",
    "crates/dotlin_codegen",
    "crates/dotlin_runtime",
    "crates/dotlin_std",
    "crates/dotlin_ffi",
    "crates/dotlin_lsp",
    "crates/dotlin_fmt",
    "crates/dotlin_test",
    "crates/dotlin_pkg",
    "crates/dotlin_diagnostics",
    "crates/dotlin_span",
    "crates/dotlin_driver",
    "crates/dotlin_macro",
    "crates/dotlin_symbol",
    "crates/dotlin_metadata",
    "tools/*",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.80"
license = "MIT OR Apache-2.0"
repository = "https://github.com/dotlin-lang/dotlin"
authors = ["Dotlin Contributors"]

[workspace.dependencies]
# Internal dependencies
dotlin_lexer = { path = "crates/dotlin_lexer" }
dotlin_parser = { path = "crates/dotlin_parser" }
dotlin_semantic = { path = "crates/dotlin_semantic" }
dotlin_mir = { path = "crates/dotlin_mir" }
dotlin_codegen = { path = "crates/dotlin_codegen" }
dotlin_runtime = { path = "crates/dotlin_runtime" }
dotlin_diagnostics = { path = "crates/dotlin_diagnostics" }
dotlin_span = { path = "crates/dotlin_span" }
dotlin_symbol = { path = "crates/dotlin_symbol" }

# External dependencies
clap = { version = "4.5", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
tracing = "0.1"
tracing-subscriber = "0.3"
ariadne = "0.4"
codespan-reporting = "0.11"
logos = "0.14"
lalrpop = "0.21"
inkwell = { version = "0.5", features = ["llvm18-0"] }
cranelift = "0.109"
cranelift-codegen = "0.109"
cranelift-frontend = "0.109"
cranelift-module = "0.109"
object = "0.36"
tower-lsp = "0.20"
lsp-types = "0.95"
tokio = { version = "1.38", features = ["full"] }
async-trait = "0.1"
dashmap = "6.0"
rayon = "1.10"
parking_lot = "0.12"
once_cell = "1.19"
crossbeam = "0.8"
indexmap = "2.2"
rustc-hash = "2.0"
petgraph = "0.6"
salsa = "0.17"

[profile.dev]
opt-level = 0
debug = true
split-debuginfo = "unpacked"

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = true
panic = "abort"

[profile.bench]
inherits = "release"

[profile.test]
opt-level = 1
```

### `rust-toolchain.toml`

```toml
[toolchain]
channel = "nightly-2024-06-01"  # Adjust for Rust 2024 availability
components = ["rustfmt", "clippy", "rust-src", "rust-analyzer"]
profile = "minimal"
```

---

## Key Crate Structures

### `crates/dotlin_cli/Cargo.toml`

```toml
[package]
name = "dotlin_cli"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "dotlinc"
path = "src/main.rs"

[dependencies]
dotlin_lexer.workspace = true
dotlin_parser.workspace = true
dotlin_semantic.workspace = true
dotlin_mir.workspace = true
dotlin_codegen.workspace = true
dotlin_driver.workspace = true
dotlin_pkg.workspace = true
dotlin_diagnostics.workspace = true

clap.workspace = true
anyhow.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
serde.workspace = true
toml.workspace = true
```

### `crates/dotlin_lexer/Cargo.toml`

```toml
[package]
name = "dotlin_lexer"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
dotlin_span.workspace = true
dotlin_diagnostics.workspace = true

logos.workspace = true
unicode-xid = "0.2"

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "lexer_bench"
harness = false
```

### `crates/dotlin_parser/Cargo.toml`

```toml
[package]
name = "dotlin_parser"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
dotlin_lexer.workspace = true
dotlin_span.workspace = true
dotlin_diagnostics.workspace = true

[build-dependencies]
lalrpop.workspace = true

[dev-dependencies]
insta = "1.39"
```

### `crates/dotlin_codegen/Cargo.toml`

```toml
[package]
name = "dotlin_codegen"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
dotlin_mir.workspace = true
dotlin_span.workspace = true

inkwell = { workspace = true, optional = true }
cranelift.workspace = true
cranelift-codegen.workspace = true
cranelift-frontend.workspace = true
cranelift-module.workspace = true
object.workspace = true

[features]
default = ["llvm"]
llvm = ["inkwell"]
cranelift-backend = []
```

### `crates/dotlin_lsp/Cargo.toml`

```toml
[package]
name = "dotlin_lsp"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "dotlin-lsp"
path = "src/main.rs"

[dependencies]
dotlin_driver.workspace = true
dotlin_parser.workspace = true
dotlin_semantic.workspace = true

tower-lsp.workspace = true
lsp-types.workspace = true
tokio.workspace = true
async-trait.workspace = true
dashmap.workspace = true
tracing.workspace = true
```

---

## Example Dotlin Project Structure

```
my_dotlin_project/
├── Dotlin.toml                 # Project manifest
├── Dotlin.lock                 # Dependency lock file
├── src/
│   ├── main.lin                # Main entry point
│   ├── lib.lin                 # Library root
│   ├── models/
│   │   ├── user.lin
│   │   └── post.lin
│   └── utils/
│       └── helpers.lin
├── tests/
│   ├── integration_test.lin
│   └── unit_test.lin
├── benches/
│   └── benchmark.lin
└── examples/
    └── example.lin
```

### `Dotlin.toml` Format

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2024"
authors = ["Your Name <you@example.com>"]

[dependencies]
dotlin_http = "1.0"
dotlin_json = "2.3"

[dev-dependencies]
dotlin_test = "1.0"

[build]
target = "native"
opt-level = 3

[features]
default = ["std"]
std = []
no-std = []
```

---

## Build System Integration

### `scripts/bootstrap.sh`

```bash
#!/bin/bash
# Bootstrap the Dotlin compiler

set -e

echo "Building Dotlin compiler..."
cargo build --release --bin dotlinc

echo "Installing compiler..."
cargo
```

---

This is what we've created. A comprehensive project structure for Dotlin using Rust 2024 edition. Here are the highlights:

## Key Features:

1. **Modular Architecture**: Separated into focused crates (lexer, parser, semantic analysis, MIR, codegen, etc.)

2. **Compiler Pipeline**:
   - `dotlin_lexer` → Tokenization
   - `dotlin_parser` → AST generation
   - `dotlin_semantic` → Type checking & analysis
   - `dotlin_mir` → Mid-level IR for optimization
   - `dotlin_codegen` → LLVM/Cranelift backend

3. **Tooling Ecosystem**:
   - CLI (`dotlin_cli`)
   - LSP server (`dotlin_lsp`)
   - Formatter (`dotlin_fmt`)
   - Package manager (`dotlin_pkg`)
   - Test framework (`dotlin_test`)

4. **Standard Library**: Located in `dotlin_std` with both Rust implementations and Dotlin source files

5. **IDE Support**: VS Code, IntelliJ IDEA, Zed and Vim integrations

6. **Modern Rust Features**:
   - Workspace configuration
   - Rust 2024 edition
   - LTO and optimization profiles
   - Incremental compilation support

The structure is designed to be scalable, maintainable, and follows Rust best practices. Each crate has its own tests and can be developed independently while maintaining clear dependency relationships.