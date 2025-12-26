# Cross-Platform Build Configuration for Dotlin

## Building for Windows

### Prerequisites
- Rust toolchain installed
- Windows SDK (for MSVC toolchain)

### Build Commands
```bash
# Install Windows targets
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc

# Build for Windows (x86_64)
cargo build --target x86_64-pc-windows-msvc --release

# Build for Windows (ARM64)
cargo build --target aarch64-pc-windows-msvc --release
```

## Building for Linux

### Prerequisites
- Rust toolchain installed
- Standard build tools (gcc, make, etc.)

### Build Commands
```bash
# Install Linux targets
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# Build for Linux (x86_64)
cargo build --target x86_64-unknown-linux-gnu --release

# Build for Linux (ARM64)
cargo build --target aarch64-unknown-linux-gnu --release
```

## Building for macOS

### Prerequisites
- Rust toolchain installed
- Xcode command line tools

### Build Commands
```bash
# Install macOS targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for macOS (Intel)
cargo build --target x86_64-apple-darwin --release

# Build for macOS (Apple Silicon)
cargo build --target aarch64-apple-darwin --release
```

## GitHub Actions Workflow

Create `.github/workflows/cross-platform-build.yml`:

```yaml
name: Cross-Platform Build

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact-name: dotlin-linux-x86_64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact-name: dotlin-windows-x86_64.exe
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact-name: dotlin-macos-x86_64

    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable
      with:
        targets: ${{ matrix.target }}
        
    - name: Install target
      run: rustup target add ${{ matrix.target }}
    
    - name: Build
      run: cargo build --target ${{ matrix.target }} --release
      
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      with:
        name: ${{ matrix.artifact-name }}
        path: target/${{ matrix.target }}/release/dotc*
```

## Cross-compilation for Linux on Windows

For building Linux binaries from Windows (requires Docker):

```bash
# Install cross utility
cargo install cross

# Build for Linux
cross build --target x86_64-unknown-linux-gnu --release
```

## Distribution Package Structure

```
dotlin-v0.1.0/
├── bin/
│   ├── dotc      # Compiler
│   ├── dotrepl   # REPL
│   └── dotfmt    # Formatter
├── lib/
│   └── dotlin_runtime.lib  # Runtime library
├── examples/
│   └── hello.lin
└── README.md
```

## Automated Release Script

Create `scripts/build-release.sh`:

```bash
#!/bin/bash

# Dotlin Release Build Script

set -e

VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
echo "Building Dotlin v$VERSION"

# Create output directory
mkdir -p "release/dotlin-v$VERSION"

# Build for current platform
cargo build --release

# Copy binaries
cp target/release/dotc "release/dotlin-v$VERSION/"
cp target/release/dotrepl "release/dotlin-v$VERSION/"
cp target/release/dotfmt "release/dotlin-v$VERSION/"
cp target/release/dotpkg "release/dotlin-v$VERSION/"
cp target/release/dotlin_lsp "release/dotlin-v$VERSION/"

# Copy examples
cp -r examples "release/dotlin-v$VERSION/examples"

# Copy documentation
cp README.md "release/dotlin-v$VERSION/"
cp -r docs "release/dotlin-v$VERSION/docs" 2>/dev/null || echo "No docs directory"

echo "Release built at release/dotlin-v$VERSION/"
```

## Windows Batch Script for Release Build

Create `scripts/build-release.bat`:

```batch
@echo off
setlocal enabledelayedexpansion

REM Dotlin Release Build Script for Windows

for /f "tokens=2,*" %%a in ('cargo metadata --no-deps --format-version 1 ^| findstr "version"') do (
    set "VERSION=%%b"
    goto :version_found
)
:version_found
set VERSION=!VERSION:"=!

echo Building Dotlin v!VERSION!

REM Create output directory
mkdir "release\dotlin-v!VERSION!" 2>nul

REM Build for current platform
cargo build --release

REM Copy binaries
copy target\release\dotc.exe "release\dotlin-v!VERSION!\" 2>nul
copy target\release\dotrepl.exe "release\dotlin-v!VERSION!\" 2>nul
copy target\release\dotfmt.exe "release\dotlin-v!VERSION!\" 2>nul
copy target\release\dotpkg.exe "release\dotlin-v!VERSION!\" 2>nul
copy target\release\dotlin_lsp.exe "release\dotlin-v!VERSION!\" 2>nul

REM Copy examples
xcopy examples "release\dotlin-v!VERSION!\examples\" /E /I /Y

REM Copy documentation
copy README.md "release\dotlin-v!VERSION!\"
xcopy docs "release\dotlin-v!VERSION!\docs\" /E /I /Y 2>nul

echo Release built at release\dotlin-v!VERSION\

endlocal
```

## Testing on Multiple Platforms

### Test Suite Execution
```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p dotlin_interpreter

# Run integration tests
cargo test --test '*'
```

### Platform-Specific Tests
Create comprehensive test files that verify all implemented features work correctly:

- Array operations (push, pop)
- HashMap operations (keys, values, size, entries)
- Type conversions (toInt, toFloat, toString)
- String operations
- Control flow
- Function calls
- etc.
```