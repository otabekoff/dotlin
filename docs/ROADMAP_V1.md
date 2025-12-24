# Dotlin v1.0 Release Roadmap

## Current Status (v0.1.0-alpha)

✅ **Completed:**
- Phase 1: Foundation & Frontend Setup
- Phase 2: Semantic Analysis & REPL
- Phase 3: Native Compilation (Backend)
- Core type system (Int, Float, String, Boolean)
- Type checking and inference
- Cranelift-based native compilation
- Basic runtime library
- VS Code syntax highlighting

## Roadmap to v1.0

### Phase 4: Standard Library & Tooling (v0.2.0 - v0.5.0)

#### 4.1 Core Standard Library (v0.2.0)
**Priority: HIGH**

##### Collections Module
- [ ] **Array Type**
  - Fixed-size arrays: `[Int; 10]`
  - Dynamic arrays/vectors: `Array<T>`
  - Methods: `push`, `pop`, `get`, `set`, `length`, `map`, `filter`, `reduce`
  - Indexing syntax: `arr[0]`
  
- [ ] **HashMap Type**
  - Generic key-value store: `Map<K, V>`
  - Methods: `insert`, `get`, `remove`, `contains`, `keys`, `values`
  - Iteration support

- [ ] **String Methods**
  - `substring(start, end)`
  - `split(delimiter)`
  - `trim()`, `toUpper()`, `toLower()`
  - `contains(substring)`
  - `replace(old, new)`

##### I/O Module
- [ ] **File Operations**
  - `readFile(path): String`
  - `writeFile(path, content): Result<(), Error>`
  - `appendFile(path, content): Result<(), Error>`
  - `fileExists(path): Boolean`
  
- [ ] **Console I/O**
  - `print(value)` - no newline
  - `println(value)` - with newline (already exists)
  - `readLine(): String`
  - `readInt(): Result<Int, Error>`

##### Math Module
- [ ] **Basic Functions**
  - `abs(x)`, `min(a, b)`, `max(a, b)`
  - `pow(base, exp)`, `sqrt(x)`
  - `floor(x)`, `ceil(x)`, `round(x)`
  - Constants: `PI`, `E`

##### Error Handling
- [ ] **Result Type**
  - `Result<T, E>` enum
  - `Ok(value)` and `Err(error)` variants
  - Methods: `isOk()`, `isErr()`, `unwrap()`, `unwrapOr(default)`

- [ ] **Option Type**
  - `Option<T>` enum
  - `Some(value)` and `None` variants
  - Methods: `isSome()`, `isNone()`, `unwrap()`, `unwrapOr(default)`

**Deliverables:**
- `dotlin_stdlib` crate
- Standard library documentation
- Example programs using stdlib

---

#### 4.2 Package Manager - `dotpkg` (v0.3.0)
**Priority: MEDIUM**

##### Core Features
- [ ] **Project Initialization**
  - `dotpkg init` - create new project
  - `dotpkg new <name>` - create new project in directory
  - Generate `Dotlin.toml` manifest

- [ ] **Dependency Management**
  - `dotpkg add <package>` - add dependency
  - `dotpkg remove <package>` - remove dependency
  - `dotpkg update` - update dependencies
  - Semantic versioning support

- [ ] **Build System**
  - `dotpkg build` - compile project
  - `dotpkg run` - compile and run
  - `dotpkg test` - run tests
  - `dotpkg clean` - clean build artifacts

- [ ] **Package Registry**
  - Central package repository (similar to crates.io)
  - `dotpkg publish` - publish package
  - `dotpkg search <query>` - search packages

##### Manifest Format (Dotlin.toml)
```toml
[package]
name = "my-project"
version = "0.1.0"
authors = ["Your Name <email@example.com>"]
edition = "2024"

[dependencies]
http = "1.0.0"
json = "0.5.0"

[dev-dependencies]
test-framework = "0.1.0"
```

**Implementation:**
- Create `dotpkg` crate
- Implement TOML parsing for manifest
- Dependency resolution algorithm
- Git-based package fetching (initially)

---

#### 4.3 Enhanced Tooling (v0.4.0)
**Priority: MEDIUM**

##### Language Server Protocol (LSP)
- [ ] **dotlin-lsp** server
  - Syntax highlighting
  - Auto-completion
  - Go-to-definition
  - Find references
  - Inline error diagnostics
  - Code formatting

##### Formatter - `dotfmt`
- [ ] **Code Formatting**
  - Consistent style enforcement
  - `dotfmt <file>` - format file
  - `dotfmt --check` - check formatting
  - Integration with editors

##### Linter - `dotlint`
- [ ] **Code Quality Checks**
  - Unused variables
  - Dead code detection
  - Style violations
  - Best practice recommendations

##### Debugger Integration
- [ ] **Debug Support**
  - DWARF debug info generation
  - GDB/LLDB integration
  - Breakpoint support
  - Variable inspection

**Deliverables:**
- `dotlin-lsp` crate
- `dotfmt` crate
- `dotlint` crate
- VS Code extension updates

---

#### 4.4 Advanced Language Features (v0.5.0)
**Priority: MEDIUM**

##### Generics
- [ ] **Generic Types**
  ```dotlin
  fun identity<T>(x: T): T {
      return x
  }
  
  class Box<T> {
      var value: T
  }
  ```

##### Traits/Interfaces
- [ ] **Trait System**
  ```dotlin
  trait Printable {
      fun toString(): String
  }
  
  class Person: Printable {
      fun toString(): String {
          return "Person"
      }
  }
  ```

##### Pattern Matching
- [ ] **Match Expression**
  ```dotlin
  match value {
      Some(x) => println(x),
      None => println("No value")
  }
  ```

##### Closures/Lambdas
- [ ] **Anonymous Functions**
  ```dotlin
  var add = |a, b| => a + b
  var numbers = [1, 2, 3].map(|x| => x * 2)
  ```

**Deliverables:**
- Extended AST for new features
- Type system enhancements
- Codegen updates
- Documentation and examples

---

### Phase 5: Distribution & Release (v0.6.0 - v1.0.0)

#### 5.1 Cross-Platform Build System (v0.6.0)
**Priority: HIGH**

##### Build Infrastructure
- [ ] **GitHub Actions CI/CD**
  - Build matrix: Windows (x64), Linux (x64, ARM64), macOS (x64, ARM64)
  - Automated testing on all platforms
  - Release artifact generation

- [ ] **Build Scripts**
  - `build.rs` for platform-specific compilation
  - Static linking of runtime library
  - Optimization flags for release builds

- [ ] **Binary Distribution**
  - Standalone executables (no Rust toolchain required)
  - Bundled runtime library
  - Minimal dependencies

##### Platform-Specific Considerations
- **Windows:**
  - MSVC runtime linking
  - Windows Installer (MSI) package
  - PATH environment variable setup

- **Linux:**
  - `.deb` packages (Debian/Ubuntu)
  - `.rpm` packages (Fedora/RHEL)
  - AppImage for universal compatibility
  - Static binary with musl

- **macOS:**
  - Universal binaries (x64 + ARM64)
  - `.dmg` installer
  - Homebrew formula
  - Code signing and notarization

**Deliverables:**
- CI/CD pipeline configuration
- Platform-specific installers
- Build documentation

---

#### 5.2 Installation System (v0.7.0)
**Priority: HIGH**

##### Installation Methods

**1. Shell Script (Linux/macOS)**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://dotlin.dev/install.sh | sh
```
- Downloads latest release
- Extracts to `~/.dotlin`
- Adds to PATH
- Verifies installation

**2. PowerShell Script (Windows)**
```powershell
iwr -useb https://dotlin.dev/install.ps1 | iex
```
- Downloads latest release
- Extracts to `%USERPROFILE%\.dotlin`
- Updates PATH
- Verifies installation

**3. Package Managers**
- **Homebrew (macOS):**
  ```bash
  brew install dotlin
  ```
  
- **Chocolatey (Windows):**
  ```powershell
  choco install dotlin
  ```
  
- **APT (Debian/Ubuntu):**
  ```bash
  sudo apt install dotlin
  ```
  
- **Cargo (Rust users):**
  ```bash
  cargo install dotlin
  ```

##### Installation Components
- `dotc` - Compiler
- `dotrepl` - REPL
- `dotpkg` - Package manager
- `dotfmt` - Formatter
- `dotlint` - Linter
- `dotlin_runtime.lib` - Runtime library
- Standard library
- Documentation

**Implementation Tasks:**
- [ ] Create installation scripts
- [ ] Set up package repositories
- [ ] Create Homebrew tap
- [ ] Submit to Chocolatey
- [ ] Create APT/RPM repositories
- [ ] Version management system

---

#### 5.3 Documentation & Examples (v0.8.0)
**Priority: HIGH**

##### Official Documentation
- [ ] **Language Guide**
  - Getting Started tutorial
  - Language syntax reference
  - Type system explanation
  - Standard library API docs
  - Best practices guide

- [ ] **Compiler Documentation**
  - Command-line options
  - Build configuration
  - Optimization flags
  - Debugging techniques

- [ ] **Package Manager Guide**
  - Creating packages
  - Publishing to registry
  - Dependency management
  - Versioning strategies

##### Example Projects
- [ ] **Basic Examples**
  - Hello World
  - Calculator
  - File I/O
  - HTTP client/server
  
- [ ] **Intermediate Examples**
  - Web scraper
  - CLI tool
  - Data processing pipeline
  - REST API

- [ ] **Advanced Examples**
  - Web framework
  - Game engine basics
  - Compiler/interpreter
  - Database driver

##### Interactive Resources
- [ ] **Online Playground**
  - Browser-based REPL
  - Code sharing
  - Example snippets
  - WebAssembly backend

- [ ] **Tutorial Series**
  - Video tutorials
  - Blog posts
  - Interactive exercises

**Deliverables:**
- Documentation website (using mdBook or similar)
- Example repository
- Online playground
- Tutorial content

---

#### 5.4 Testing & Quality Assurance (v0.9.0)
**Priority: HIGH**

##### Test Suite
- [ ] **Unit Tests**
  - Lexer tests
  - Parser tests
  - Type checker tests
  - Codegen tests
  - Runtime tests

- [ ] **Integration Tests**
  - End-to-end compilation
  - Standard library tests
  - Cross-platform compatibility
  - Performance benchmarks

- [ ] **Fuzzing**
  - Lexer fuzzing
  - Parser fuzzing
  - Type checker fuzzing
  - Crash detection

##### Quality Metrics
- [ ] **Code Coverage**
  - Target: >80% coverage
  - Coverage reports in CI
  
- [ ] **Performance Benchmarks**
  - Compilation speed
  - Runtime performance
  - Memory usage
  - Comparison with other languages

- [ ] **Compatibility Testing**
  - Windows 10/11
  - Ubuntu 20.04/22.04/24.04
  - macOS 12/13/14
  - Various architectures

**Deliverables:**
- Comprehensive test suite
- Benchmark suite
- Quality metrics dashboard
- Performance reports

---

#### 5.5 v1.0 Release (v1.0.0)
**Priority: CRITICAL**

##### Release Checklist
- [ ] **Feature Complete**
  - All planned features implemented
  - Standard library complete
  - Package manager functional
  - Tooling ecosystem ready

- [ ] **Quality Assurance**
  - All tests passing
  - No critical bugs
  - Performance acceptable
  - Documentation complete

- [ ] **Distribution Ready**
  - Installers for all platforms
  - Package manager submissions
  - Website live
  - Download mirrors set up

- [ ] **Community Ready**
  - GitHub repository public
  - Issue tracker set up
  - Contributing guidelines
  - Code of conduct
  - License (MIT/Apache 2.0)

##### Release Artifacts
- Source code (GitHub release)
- Binary distributions (all platforms)
- Package manager packages
- Documentation website
- Release notes
- Migration guide (if applicable)

##### Launch Activities
- [ ] **Announcement**
  - Blog post
  - Social media
  - Reddit/HackerNews
  - Developer communities

- [ ] **Marketing**
  - Website launch
  - Demo videos
  - Comparison benchmarks
  - Use case examples

- [ ] **Community Building**
  - Discord/Slack server
  - Forum/discussion board
  - Contribution guide
  - Roadmap for v2.0

**Deliverables:**
- v1.0.0 release
- Public announcement
- Complete documentation
- Active community channels

---

## Timeline Estimate

| Phase | Version | Duration | Target Date |
|-------|---------|----------|-------------|
| 4.1 - Core Stdlib | v0.2.0 | 4-6 weeks | Feb 2025 |
| 4.2 - Package Manager | v0.3.0 | 3-4 weeks | Mar 2025 |
| 4.3 - Enhanced Tooling | v0.4.0 | 4-6 weeks | Apr 2025 |
| 4.4 - Advanced Features | v0.5.0 | 6-8 weeks | Jun 2025 |
| 5.1 - Cross-Platform Build | v0.6.0 | 2-3 weeks | Jul 2025 |
| 5.2 - Installation System | v0.7.0 | 2-3 weeks | Jul 2025 |
| 5.3 - Documentation | v0.8.0 | 3-4 weeks | Aug 2025 |
| 5.4 - Testing & QA | v0.9.0 | 3-4 weeks | Sep 2025 |
| 5.5 - v1.0 Release | v1.0.0 | 2 weeks | Oct 2025 |

**Total Estimated Time: 8-10 months**

---

## Priority Matrix

### Must Have (v1.0)
- ✅ Core language features
- ✅ Type system
- ✅ Native compilation
- Standard library (collections, I/O, math)
- Package manager
- Cross-platform installers
- Documentation

### Should Have (v1.0)
- LSP server
- Formatter
- Linter
- Generics
- Error handling (Result/Option)
- Online playground

### Nice to Have (v1.1+)
- Advanced pattern matching
- Async/await
- WebAssembly backend
- JIT compilation
- Incremental compilation

---

## Next Immediate Steps

1. **Start Phase 4.1: Core Standard Library**
   - Design Array/Vector API
   - Implement basic collections
   - Add file I/O support
   - Create math module

2. **Set Up Distribution Infrastructure**
   - Create GitHub Actions workflows
   - Set up build matrix
   - Create installation scripts
   - Register domain (dotlin.dev)

3. **Community Preparation**
   - Create CONTRIBUTING.md
   - Set up issue templates
   - Create CODE_OF_CONDUCT.md
   - Prepare announcement materials

Would you like me to start implementing any specific phase?
