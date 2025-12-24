# Project Organization - Dotlin

## Directory Structure

```
dotlin/
├── .github/                    # GitHub configuration
│   └── workflows/              # CI/CD workflows
│       └── ci.yml              # Main CI/CD pipeline
│
├── crates/                     # Rust workspace crates
│   ├── dotlin_ast/             # Abstract Syntax Tree definitions
│   ├── dotlin_lexer/           # Tokenization and lexical analysis
│   ├── dotlin_parser/          # Parser implementation
│   ├── dotlin_typechecker/     # Type checking and inference
│   ├── dotlin_codegen/         # Code generation (Cranelift backend)
│   ├── dotlin_runtime/         # Runtime library (native functions)
│   ├── dotlin_interpreter/     # Tree-walk interpreter for REPL
│   ├── dotlin_stdlib/          # Standard library (in development)
│   ├── dotc/                   # Compiler CLI tool
│   └── dotrepl/                # Interactive REPL tool
│
├── docs/                       # Documentation
│   ├── language-guide/         # Language reference and guide
│   ├── api/                    # API documentation
│   ├── tutorials/              # Tutorials and how-to guides
│   ├── README.md               # Documentation index
│   ├── ROADMAP_V1.md           # Roadmap to v1.0 release
│   ├── STATUS.md               # Current implementation status
│   ├── PROGRESS.md             # Development progress tracking
│   └── implementation_plan.md  # Original implementation plan
│
├── examples/                   # Example programs
│   ├── basic/                  # Basic examples (*.lin files)
│   │   ├── hello.lin           # Hello World
│   │   ├── test_*.lin          # Various test examples
│   │   └── ...
│   ├── intermediate/           # Intermediate examples
│   └── advanced/               # Advanced examples
│
├── tests/                      # Test suite
│   ├── integration/            # Integration tests
│   └── unit/                   # Unit tests
│
├── scripts/                    # Build and installation scripts
│   ├── install.sh              # Unix/Linux/macOS installer
│   └── install.ps1             # Windows PowerShell installer
│
├── editors/                    # Editor support
│   └── vscode/                 # VS Code extension
│       └── dotlin-syntax/      # Syntax highlighting
│
├── lib/                        # Compiled runtime libraries
│   └── dotlin_runtime.lib      # Runtime library (platform-specific)
│
├── build/                      # Build artifacts (gitignored)
│   ├── *.exe                   # Compiled executables
│   ├── *.o                     # Object files
│   ├── *.pdb                   # Debug symbols
│   └── ...
│
├── .gitignore                  # Git ignore rules
├── Cargo.toml                  # Workspace configuration
├── Cargo.lock                  # Dependency lock file
├── README.md                   # Project README
├── CONTRIBUTING.md             # Contribution guidelines
├── CHANGELOG.md                # Version history
├── LICENSE-MIT                 # MIT License
├── LICENSE-APACHE              # Apache 2.0 License
├── AGENTS.md                   # AI agents documentation
├── NOTES.md                    # Development notes
├── PROMPTS.md                  # Development prompts
└── TODO.md                     # TODO list
```

## File Organization Guidelines

### Source Code (`crates/`)

Each crate has a specific purpose:

- **dotlin_ast**: Core AST types, no dependencies on other Dotlin crates
- **dotlin_lexer**: Depends on logos, produces tokens
- **dotlin_parser**: Depends on lexer and AST, produces AST
- **dotlin_typechecker**: Depends on AST, annotates AST with types
- **dotlin_codegen**: Depends on AST, produces object code via Cranelift
- **dotlin_runtime**: Standalone, provides C-compatible runtime functions
- **dotlin_interpreter**: Depends on AST, evaluates AST directly
- **dotlin_stdlib**: Standard library implementation (in progress)
- **dotc**: Compiler binary, orchestrates all components
- **dotrepl**: REPL binary, uses interpreter

### Documentation (`docs/`)

- **language-guide/**: Complete language reference
  - Syntax
  - Type system
  - Standard library
  - Best practices

- **api/**: API documentation
  - Standard library API
  - Compiler API
  - Runtime API

- **tutorials/**: Step-by-step guides
  - Getting started
  - Common tasks
  - Advanced topics

- **Root docs files**:
  - `README.md`: Documentation index
  - `ROADMAP_V1.md`: v1.0 development roadmap
  - `STATUS.md`: Current implementation status
  - `PROGRESS.md`: Progress tracking
  - `implementation_plan.md`: Original plan

### Examples (`examples/`)

- **basic/**: Simple, beginner-friendly examples
  - Hello World
  - Variables and types
  - Functions
  - Control flow
  - String operations

- **intermediate/**: More complex examples
  - File I/O
  - Data structures
  - Algorithms
  - Error handling

- **advanced/**: Advanced use cases
  - Complex applications
  - Performance optimization
  - Design patterns

### Tests (`tests/`)

- **integration/**: End-to-end tests
  - Compilation tests
  - Runtime tests
  - Cross-platform tests

- **unit/**: Unit tests (also in `crates/*/src/`)
  - Per-crate unit tests
  - Component tests

### Scripts (`scripts/`)

- **install.sh**: Unix/Linux/macOS installation script
- **install.ps1**: Windows PowerShell installation script
- Future: build scripts, release scripts, etc.

### Build Artifacts (`build/`)

**Note**: This directory is gitignored

- Compiled executables (*.exe)
- Object files (*.o)
- Debug symbols (*.pdb)
- Temporary build files
- Test outputs

## File Naming Conventions

### Rust Files
- Use snake_case for file names
- Match module names
- Example: `type_checker.rs`, `code_generator.rs`

### Dotlin Files
- Use snake_case for file names
- Extension: `.lin`
- Example: `hello_world.lin`, `file_io.lin`

### Documentation Files
- Use UPPERCASE for root-level docs
- Use Title Case for subdirectory docs
- Extension: `.md`
- Example: `README.md`, `CONTRIBUTING.md`, `Getting-Started.md`

### Test Files
- Prefix with `test_` for Dotlin test files
- Use descriptive names
- Example: `test_string_operations.lin`, `test_type_checking.lin`

## Maintenance Guidelines

### Adding New Features

1. **Update AST** (`dotlin_ast`) if needed
2. **Update Lexer** (`dotlin_lexer`) for new tokens
3. **Update Parser** (`dotlin_parser`) for new syntax
4. **Update Type Checker** (`dotlin_typechecker`) for type rules
5. **Update Codegen** (`dotlin_codegen`) for code generation
6. **Update Runtime** (`dotlin_runtime`) for runtime support
7. **Update Interpreter** (`dotlin_interpreter`) for REPL
8. **Add Tests** in appropriate test directory
9. **Add Examples** in `examples/`
10. **Update Documentation** in `docs/`
11. **Update CHANGELOG.md**

### Cleaning Up

```bash
# Remove build artifacts
cargo clean

# Remove all generated files
rm -rf build/
rm -rf target/

# Remove test outputs
rm -rf test_output/
```

### Before Committing

1. Run tests: `cargo test`
2. Run clippy: `cargo clippy`
3. Format code: `cargo fmt`
4. Update documentation
5. Update CHANGELOG.md
6. Check .gitignore

## Current Organization Status

### ✅ Completed

- [x] Created proper directory structure
- [x] Moved documentation to `docs/`
- [x] Moved examples to `examples/basic/`
- [x] Moved build artifacts to `build/`
- [x] Created comprehensive README.md
- [x] Created CONTRIBUTING.md
- [x] Created CHANGELOG.md
- [x] Created .gitignore
- [x] Organized crates in workspace

### 🚧 In Progress

- [ ] Populate `examples/intermediate/`
- [ ] Populate `examples/advanced/`
- [ ] Create language guide in `docs/language-guide/`
- [ ] Create API docs in `docs/api/`
- [ ] Create tutorials in `docs/tutorials/`
- [ ] Add integration tests in `tests/integration/`

### 📋 Planned

- [ ] Set up documentation website
- [ ] Create benchmark suite
- [ ] Add more comprehensive examples
- [ ] Create video tutorials
- [ ] Set up online playground

## Quick Reference

### Common Commands

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run compiler
cargo run -p dotc -- input.lin -o output

# Run REPL
cargo run -p dotrepl

# Format code
cargo fmt

# Run linter
cargo clippy

# Clean build
cargo clean
```

### File Locations

- **Source code**: `crates/*/src/`
- **Tests**: `crates/*/tests/` or `tests/`
- **Examples**: `examples/`
- **Documentation**: `docs/`
- **Build output**: `build/` (gitignored)
- **Cargo output**: `target/` (gitignored)

---

**Last Updated**: December 24, 2024
**Maintainer**: Dotlin Team
