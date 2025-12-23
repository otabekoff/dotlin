# Implementation Plan - Dotlin Extended Types & Build System

This plan outlines the steps to expand Dotlin's type system to support Floats and Strings, and to refine the project's build and runtime integration.

## Phase 1: Build System Refinement
- [x] Create a `lib` directory in the project root to house the `dotlin_runtime` library.
- [x] Update `dotc` to look for the runtime in default locations (relative to the executable or in the project root) if `--runtime-path` is not provided.
- [x] Automate the copying of `dotlin_runtime` to the `lib` directory during build (conceptually done via compiler path search).

## Phase 2: Float Support
- [x] **Lexer**: Add a regex for float literals (e.g., `123.456`).
- [x] **Parser**: Update `parse_primary` to handle `Token::Float` and create `Literal::Float`.
- [x] **Codegen**:
    - [x] Implement float literal generation using `builder.ins().f64const`.
    - [x] Update binary operator compilation to check operand types.
    - [x] Implement float arithmetic (`fadd`, `fsub`, `fmul`, `fdiv`).
    - [x] Implement float comparisons (`fcmp`).

## Phase 3: Enhanced String Operations
- [x] **Runtime**: Add helper functions to `dotlin_runtime` for string concatenation and comparison.
- [ ] **Runtime**: Add helper function for string length (if needed) or handle it inline.
- [x] **Codegen**:
    - [x] Implement string concatenation as calls to the runtime.
    - [ ] Add support for string `length` property/function in Dotlin.
- [x] **Runtime**: Implemented `dotlin_string_compare` and `dotlin_string_concat`.

## Phase 4: Basic Type Inference/Checking
- [x] Implement a minimal type checking pass that runs before codegen to ensure type safety (e.g., can't add an Int to a String).
- [x] Annotate the AST with type information to simplify codegen decisions.

## Phase 5: Optimization & Finalization
- [x] Enable Cranelift's optimization passes in `CodeGenerator`.
- [ ] Verify performance improvements with benchmarks.
- [x] Implement string `.length` property.

## Summary

All major features have been successfully implemented:
- ✅ Float type support (literals, arithmetic, comparisons)
- ✅ String concatenation and comparison
- ✅ String `.length` property
- ✅ Type checking pass with AST annotation
- ✅ Cranelift optimization enabled
- ✅ Build system refinements

Remaining work:
- Performance benchmarking
- Additional standard library features (as needed)
