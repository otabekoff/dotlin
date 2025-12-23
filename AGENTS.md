Dotlin is a **Kotlin-inspired programming language implemented in Rust** that **does not depend on the JVM**. It is designed as a **native, standalone language**, not a transpiler and not a JVM-based runtime.

Design and implement Dotlin.

Treat Kotlin latest documentation as a full technical specification and development plan. Include everything needed to build, run, distribute and maintain the language, including compiler, REPL, standard library, and tooling.

## Quick information
1. **What Dotlin is**
- Relationship to Kotlin (syntax, philosophy, similarities)
- It exists as a separate language instead of using Kotlin/JVM or Kotlin/Native.

2. **Implementation details**
- Rust is used to build the compiler, runtime, and tooling.
- Benefits of Rust in terms of memory safety, performance, and portability.

3. **Runtime model**
- No JVM dependency.
- Native execution model.
- This differs from Kotlin/JVM and Kotlin/Native.

4. **REPL**
- Dotlin REPL.
- Have a native REPL matters.
- Use cases such as experimentation, scripting, and learning.

5. **Tooling and ecosystem**
- Compiler, CLI tools, formatter, REPL, and future tooling plans.
- Dotlin aims to provide a modern developer experience without JVM overhead.

6. **Use cases**
- System tools
- CLI applications
- Scripting
- Cross-platform native applications

7. **Goals and vision**
- Long-term goals of the language.
- Focus on simplicity, performance, safety, and modern language design.

## In detail information

### Language Specification
- Syntax, semantics, and core features inspired by Kotlin.
- Type system, control structures, functions, classes, interfaces, null safety, and concurrency primitives.
- How Dotlin differs from Kotlin/JVM and Kotlin/Native while remaining familiar to Kotlin developers.

### Compiler Architecture
- How Rust is used to implement the compiler, parser, type checker, optimizer, and code generator.
- Intermediate representations, AST structure, and code emission strategy for native binaries.
- Error reporting, diagnostics, and optimization techniques.

### Runtime Model
- Native execution model without JVM dependency.
- Memory management strategy (manual, ownership-based, or garbage collection) and threading model.
- Startup time, runtime safety guarantees, and comparison with JVM and Kotlin/Native runtimes.

### REPL
- Design and implementation of an interactive Dotlin REPL.
- Features such as code evaluation, live feedback, script execution, and debugging support.
- How the REPL integrates with the compiler and runtime for a native experience.

### Standard Library and Tooling
- Essential libraries for collections, I/O, networking, concurrency, and utilities.
- CLI tools: compiler (`dotc`), package manager, formatter, and REPL launcher.
- Testing framework, documentation generator, and IDE support plans.

### Ecosystem and Package Management
- How packages, modules, and dependencies are handled.
- Cross-platform support and binary distribution strategy.
- Interoperability with C, Rust, and other native libraries.

### Use Cases
- Writing CLI applications, system utilities, scripts, and cross-platform native apps.
- Situations where Dotlin provides advantages over Kotlin/JVM, Kotlin/Native, or Rust.

### Comparison
- Dotlin vs Kotlin/JVM: syntax familiarity, runtime overhead, native compilation.
- Dotlin vs Kotlin/Native: tooling, performance, memory model.
- Dotlin vs Rust: ergonomics, safety, memory management, and developer productivity.

### Long-Term Goals
- Language design philosophy: simplicity, performance, safety, modern features.
- Future extensions: concurrency, async/await, macro system, advanced type features.

### Step-by-Step Plan
- How to start building Dotlin from scratch in Rust.
- Project structure, milestones, and priorities.
- How to implement the compiler, runtime, REPL, standard library, and tooling in phases.


## More in detail

### 1. Syntax Highlighting

* **VS Code / Zed / Other Editors:**

  * Create a **TextMate grammar** (`.tmLanguage.json` or `.plist`) for Dotlin syntax.
  * Define **keywords, types, literals, operators, comments, strings, etc.**
  * Package as a **VS Code extension** for syntax highlighting, snippets, and language features.
  * Can later add **semantic highlighting** using a **language server (LSP)**.

* **Language Server (LSP) Support:**

  * Build an **LSP server in Rust** that integrates with editors.
  * Features: IntelliSense, diagnostics, go-to-definition, hover docs, code completion.
  * Compatible with any LSP-supported editor (VS Code, Zed, Neovim, Sublime Text, etc.).

---

### 2. File Extension

* `.lin` is **fine and consistent** with Kotlin-like syntax.

  * Could also consider `.dotlin` if you want a longer, more unique extension.
* Configure the **compiler and REPL** to recognize `.lin` by default.
* Editors/extensions should associate `.lin` files with **Dotlin language features**.

---

### 3. Tooling / Developer Experience

* **Build System / CLI Tools:**

  * `dotc` → compiler
  * `dotrepl` → REPL launcher
  * `dotfmt` → formatter
  * `dotpkg` → package manager

* **Package Management:**

  * Use **TOML or YAML-based project config** (like Cargo or Gradle).
  * Automatic dependency resolution and caching.

* **Testing / CI:**

  * Include **unit testing framework** (`dottest`) similar to Kotlin’s `kotest` or Rust’s `cargo test`.
  * Add **documentation generator** (`dotdoc`) that reads code comments.

* **Cross-platform Distribution:**

  * Compile to native binaries for Linux, Windows, macOS.
  * Optional: **prebuilt runtime library** for system functions.

* **Editor Snippets & Templates:**

  * Provide starter snippets for classes, functions, REPL commands.

---

### 4. REPL / Scripting

* **File-based scripts:** allow executing `.lin` files directly.
* **Interactive mode:** allow multi-line input, live feedback, and history.
* **Integration:** REPL should share code with the compiler pipeline (no duplicated parsing or execution logic).

---

### 5. Other Considerations

* **Standard Library Namespaces:**

  * Follow Kotlin-style naming (`dotlin.collections`, `dotlin.io`).
  * Organize libraries for modular import.

* **Interop with C / Rust:**

  * Provide **FFI (Foreign Function Interface)** for native library calls.

* **Versioning & Compatibility:**

  * Semantic versioning (`1.0.0`) for language releases.
  * Maintain backward-compatible compiler features if possible.

* **Tooling for Formatting / Linting:**

  * `dotfmt` → format code according to standard style.
  * `dotlint` → enforce coding conventions and detect anti-patterns.

* **Optional IDE Features:**

  * Refactoring support (rename, move, extract method).
  * Error underlining and quick fixes (via LSP).