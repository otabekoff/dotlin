# Dotlin Implementation Status - December 25, 2025
### 📋 **Roadmap to v1.0**

#### v0.3.0: Package Manager (March 2025)
- [x] `dotpkg` CLI tool for dependency management
- [x] Project initialization and manifest format
- [ ] Package registry infrastructure

#### v0.4.0: Enhanced Tooling (April 2025)
- [x] Interpreter implementation with full feature support
- [x] Incremental build system optimization
- [x] Math functions implementation (abs, min, max, sqrt, etc.)
- [ ] Language Server Protocol (LSP) support
- [ ] Code formatter (`dotfmt`) and linter (`dotlint`)
- [ ] Debugger integration

#### v0.5.0: Advanced Features (June 2025)
- [ ] Generic types and functions (Required for advanced HashMap usage)
- [ ] Trait system for interfaces
- [ ] Pattern matching with `match` expressions
- [ ] Closures and anonymous functions

#### v0.5.5-v0.6.0: Distribution & Release Preparation (March-April 2025)
- [x] Cross-platform build system configuration
- [ ] Installation methods (scripts, package managers)
- [ ] Comprehensive documentation
- [ ] Public release with full tooling

# Dotlin Implementation Status — Remaining Work (Dec 26, 2025)

This file now lists only outstanding items and next steps to keep the roadmap focused.

## Remaining High-Priority Work

- Implement proper codegen lowering for iterator objects
    - Runtime: iterator object representation and ownership rules
    - Codegen: lower `obj.iter()` to create iterator object and `.next()` to advance it
    - Tests: integration test compiling `for (k, v) in m.iter()` and executing the binary end-to-end

- Add integration tests and CI coverage for iterator/codegen/runtime
    - End-to-end test that compiles a small program using `m.iter()` and runs it
    - Unit tests for `.next()` behavior and exhaustion (interpreter tests already added)

- Documentation & examples
    - Formatter examples and docs demonstrating `for (k, v) in m.iter()` and `.next()` usage
    - Update examples/ and docs/ with iterator samples

## Longer-term / Nice-to-have

- LSP support and formatter/linter improvements
- Generic types and trait system
- Pattern matching and advanced language features

---
Updated: December 26, 2025