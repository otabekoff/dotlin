INSTRUCTIONS — Dotlin project layout
=================================

> To run: `cargo run -p dotlin_cli -- run examples/hello_world/main.lin one two three`

This document describes the workspace layout and the responsibility of each folder and key files. I inspected each top-level folder and the `crates/` folder; where a crate contains source files I also listed each source file's intended purpose.

Root (top-level)
- `Cargo.toml` — workspace manifest: lists workspace members, shared workspace package metadata and profiles. Controls which crates are built together.
- `Cargo.lock` — lockfile for reproducible builds.
- `README.md` — high-level project overview, goals, language characteristics and usage notes.
- `STRUCTURE.md` — repository structure and intended layout for contributors.
- `CHANGELOG.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `TODO.md` — project governance and planning docs.
- `LICENSE-APACHE`, `LICENSE-MIT` — licenses.
- `rust-toolchain.toml` — recommended Rust toolchain settings.
- `src/main.rs` — a tiny root binary (placeholder) that prints "Hello, world!"; not the compiler itself (compiler lives in `crates/`).

Top-level folders
- `crates/` — main workspace crates (see crates section below).
- `docs/` — documentation for the compiler and language. Important subfolders:
  - `docs/language-spec/` — language specification chapters: `01-syntax.md`, `02-type-system.md`, `03-memory-model.md`, `04-concurrency.md`, `05-interop.md`. We added a "Comments" section to `01-syntax.md` describing single-line and nested block comments.
  - `docs/compiler/` — compiler architecture and passes (e.g., `passes.md`, `optimization.md`).
  - `docs/tutorials/` — user-facing tutorials.
- `examples/` — sample Dotlin projects (e.g., `hello_world`, `ffi_example`, `http_server`, `concurrent_processing`). Each contains `.lin` example source.
- `stdlib_impl/` — implementation of the standard library in Dotlin (organization by module: `collections/`, `core/`, `io/`).
- `tools/` — helper tools for the project (e.g., `perf_analyzer`, `syntax_gen`, `test_runner`).
- `scripts/` — developer scripts used for automation, CI helpers, etc.
- `benches/` — benchmarks used with `cargo bench`.
- `tests/` — integration tests and fixtures used by the project.

Crates (under `crates/`)
---------------------------------
Note: Some crates are currently minimal/placeholder or empty in this workspace snapshot. For each crate I list the crate purpose and the files currently present.

- `crates/dotlin_cli/` — Command-line interface for Dotlin.
  - `Cargo.toml` — binary crate manifest; defines the `dotlinc` binary.
  - `src/main.rs` — CLI implementation using `clap`; provides `dotlinc hello` placeholder and subcommand scaffolding. In a full compiler this is the entrypoint wiring together driver, parser, and codegen.

- `crates/dotlin_codegen/` — Code generation backends (LLVM/Cranelift) and lowering from MIR to machine code.
  - `Cargo.toml` — simplified/placeholder in this workspace.
  - `src/lib.rs` — placeholder `placeholder_codegen()` function present to allow workspace builds. In a full project this crate would host backend selection and machine-code emission.

- `crates/dotlin_diagnostics/` — (empty folder) Intended: diagnostics reporting utilities (error types, diagnostic formatting, spans -> messages). Currently empty or not included in this snapshot.

- `crates/dotlin_driver/` — (empty folder) Intended: top-level driver which orchestrates compilation phases (parse -> semantic -> mir -> codegen -> link). Currently empty.

- `crates/dotlin_ffi/` — (empty) Intended: FFI helpers for interop with C/C++ and dynamic linking.

- `crates/dotlin_fmt/` — (empty) Intended: formatting tool (like rustfmt) for Dotlin source.

- `crates/dotlin_lexer/` — Lexer / scanner for Dotlin. This crate now contains an integrated lexer and tests for nested comments.
  - `Cargo.toml` — crate manifest (self-contained for local testing).
  - `src/lib.rs` — module exports for the lexer; re-exports `lex` and `Token`.
  - `src/cursor.rs` — small `Cursor` abstraction providing `peek`, `next` and utilities for scanning characters. Used by the streaming lexer.
  - `src/lexer.rs` — streaming lexer implementation: produces basic `Token` values, handles skipping single-line and nested block comments, recognizes strings and character literals and basic identifiers/numbers/symbols. This is a distilled implementation to be integrated into a more complete lexer with spans and logos integration later.
  - `src/token.rs` — `Token` enum (minimal) used for tests: `Ident`, `Number`, `Str`, `Char`, `Symbol`.
  - `src/error.rs` — intended for lexing error types (currently empty placeholder).
  - `src/keywords.rs` — place for keyword table / recognition (placeholder).
  - `tests/lexer_tests.rs` — unit tests validating nested block comments, comment delimiters inside strings/char literals, and line comments.
  - `tests/token_tests.rs` — token tests (currently empty).

- `crates/dotlin_lsp/` — Language Server Protocol integration.
  - `Cargo.toml` — defines `dotlin-lsp` binary and depends on `tower-lsp`, tokio, and workspace crates (driver/parser/semantic) when present.
  - `src/` — (not present in snapshot) would implement LSP features: diagnostics, completion, go-to-definition, etc.

- `crates/dotlin_macro/` — (empty) Intended: procedural macros for Dotlin (if any).

- `crates/dotlin_metadata/` — (empty) Intended: metadata storage and serialization between compilation phases or for package metadata.

- `crates/dotlin_mir/` — (empty) Intended: Mid-level intermediate representation (MIR) definitions, transformations and analyses.

- `crates/dotlin_parser/` — Parser for Dotlin source.
  - `Cargo.toml` — depends on `dotlin_lexer` and `lalrpop` build dependency.
  - `src/ast/` — AST node definitions (folder present).
  - `src/grammar/` — parser grammar files (LALRPOP) and generated code.
  - `src/parser.rs` — parser entry points.
  - `src/precedence.rs` — operator precedence table/helpers.
  - `src/span.rs` — span types for source locations.
  - `src/error.rs` — parser error types.
  - `src/tests/` — parser tests/fixtures.

- `crates/dotlin_pkg/` — (empty) Package manager / packaging utilities (intended).

- `crates/dotlin_runtime/` — (empty) Runtime libraries and support (garbage collection, task scheduler, std runtime functions).

- `crates/dotlin_semantic/` — (empty) Semantic analysis (type checking, name resolution, symbol tables).

- `crates/dotlin_span/` — (empty) Source span and position utilities (a lightweight crate might hold common span types used across crates).

- `crates/dotlin_std/` — (empty) Standard library built-in modules (core runtime glue for the standard library).

- `crates/dotlin_symbol/` — (empty) Symbol table implementations (AST -> symbol resolution helpers).

- `crates/dotlin_test/` — (empty) Testing harnesses for compiled Dotlin code and test runners.

Other folders explained
- `docs/` — documentation; see above. Files inside `docs/language-spec/` correspond to language chapters.
- `examples/` — small Dotlin example programs (empty `.lin` files present in snapshot or placeholders).
- `stdlib_impl/` — implementation of runtime stdlib modules in Dotlin source (organization by subject).
- `tools/` — scripts and binaries to aid development (`perf_analyzer`, `syntax_gen`, `test_runner`).
- `scripts/` — assorted helper scripts used by contributors.
- `benches/` — benchmarking entries (`compiler_bench.rs`, `runtime_bench.rs`, `stdlib_bench.rs`).
- `tests/` — integration tests and fixtures (e.g., `integration/compile_tests.rs`).

Notes and recommended next actions
- Many crates are currently placeholders or empty. When restoring the full workspace, each crate's `Cargo.toml` should be re-enabled to reference workspace dependencies (e.g., `dotlin_lexer` as a path dependency) and the workspace `Cargo.toml` members list should be restored.
- The `dotlin_lexer` crate now contains a working streaming lexer module that correctly handles nested block comments and preserves comment-like text inside string/char literals. To integrate it into the full compiler, replace the placeholder `logos`-based scanner (if present) or adapt the `Cursor` and `lexer` to produce `Token` values with spans.
- I can continue by expanding each placeholder crate in `crates/` with minimal `Cargo.toml` + `src/lib.rs` placeholders (so the full workspace becomes buildable), or integrate the lexer into the full pipeline (parser -> semantic -> mir).

If you want, I will now:
1) Expand `INSTRUCTIONS.md` with a per-file line inside each crate (I can append more granular file-level descriptions), or
2) Start restoring additional crates as minimal placeholders to allow building the entire workspace, or
3) Integrate `dotlin_lexer` tokens/spans into `crates/dotlin_parser` and run parser tests.

— End of INSTRUCTIONS.md
