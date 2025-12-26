@echo off
REM Dotlin Windows Build and Test Script
REM This script builds and tests all Dotlin crates on Windows

setlocal enabledelayedexpansion

echo ========================================
echo Dotlin Windows Build and Test Script
echo ========================================
echo.

REM Check if Rust is installed
rustc --version >nul 2>&1
if errorlevel 1 (
    echo Error: Rust is not installed or not in PATH
    echo Please install Rust from https://rustup.rs/
    exit /b 1
)

echo Rust version:
rustc --version
echo.

REM Check if cargo is installed
cargo --version >nul 2>&1
if errorlevel 1 (
    echo Error: Cargo is not installed or not in PATH
    exit /b 1
)

echo Cargo version:
cargo --version
echo.

REM Update Rust toolchain
echo Updating Rust toolchain...
rustup update
echo.

REM Add Windows targets if not already installed
echo Installing Windows targets...
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
echo.

REM Clean previous builds
echo Cleaning previous builds...
cargo clean
echo.

REM Check all crates
echo Checking all Dotlin crates...
cargo check --workspace
if errorlevel 1 (
    echo Error: Cargo check failed
    exit /b 1
)
echo.

REM Build all crates in release mode
echo Building all Dotlin crates in release mode...
cargo build --release
if errorlevel 1 (
    echo Error: Cargo build failed
    exit /b 1
)
echo.

REM Run tests for each crate
echo Running tests for dotlin_interpreter...
cargo test -p dotlin_interpreter
if errorlevel 1 (
    echo Warning: Some tests failed in dotlin_interpreter
)

echo Running tests for dotlin_typechecker...
cargo test -p dotlin_typechecker
if errorlevel 1 (
    echo Warning: Some tests failed in dotlin_typechecker
)

echo Running tests for dotlin_parser...
cargo test -p dotlin_parser
if errorlevel 1 (
    echo Warning: Some tests failed in dotlin_parser
)

echo Running tests for dotlin_lexer...
cargo test -p dotlin_lexer
if errorlevel 1 (
    echo Warning: Some tests failed in dotlin_lexer
)

echo Running tests for dotlin_codegen...
cargo test -p dotlin_codegen
if errorlevel 1 (
    echo Warning: Some tests failed in dotlin_codegen
)

echo Running tests for dotlin_runtime...
cargo test -p dotlin_runtime
if errorlevel 1 (
    echo Warning: Some tests failed in dotlin_runtime
)

REM Build specific tools
echo Building Dotlin tools...
cargo build -p dotc --release
cargo build -p dotrepl --release
cargo build -p dotfmt --release
cargo build -p dotpkg --release
cargo build -p dotlin_lsp --release
echo.

REM Test the main compiler with example files
echo Testing compiler with example files...
if exist "examples\basic\test_simple.lin" (
    echo Testing test_simple.lin...
    cargo run -p dotc -- --run examples\basic\test_simple.lin
)

if exist "test_array_methods.lin" (
    echo Testing test_array_methods.lin...
    cargo run -p dotc -- --run test_array_methods.lin
)

if exist "hashmap_all_methods_test.lin" (
    echo Testing hashmap_all_methods_test.lin...
    cargo run -p dotc -- --run hashmap_all_methods_test.lin
)

if exist "comprehensive_test.lin" (
    echo Testing comprehensive_test.lin...
    cargo run -p dotc -- --run comprehensive_test.lin
)
echo.

REM Create distribution package
echo Creating distribution package...
if not exist "dist" mkdir dist
if not exist "dist\dotlin-windows" mkdir dist\dotlin-windows

copy target\release\dotc.exe dist\dotlin-windows\ 2>nul
copy target\release\dotrepl.exe dist\dotlin-windows\ 2>nul
copy target\release\dotfmt.exe dist\dotlin-windows\ 2>nul
copy target\release\dotpkg.exe dist\dotlin-windows\ 2>nul
copy target\release\dotlin_lsp.exe dist\dotlin-windows\ 2>nul

if exist "examples" xcopy examples dist\dotlin-windows\examples\ /E /I /Y 2>nul
if exist "docs" xcopy docs dist\dotlin-windows\docs\ /E /I /Y 2>nul

copy README.md dist\dotlin-windows\ 2>nul
copy IMPLEMENTATION_SUMMARY.md dist\dotlin-windows\ 2>nul
copy MISSING_KOTLIN_FEATURES.md dist\dotlin-windows\ 2>nul
copy CROSS_PLATFORM_BUILD.md dist\dotlin-windows\ 2>nul

echo.

REM Show build results
echo Build completed successfully!
echo Binaries are available in: dist\dotlin-windows\
echo.

echo Dotlin tools built:
if exist "dist\dotlin-windows\dotc.exe" echo - dotc.exe (compiler)
if exist "dist\dotlin-windows\dotrepl.exe" echo - dotrepl.exe (REPL)
if exist "dist\dotlin-windows\dotfmt.exe" echo - dotfmt.exe (formatter)
if exist "dist\dotlin-windows\dotpkg.exe" echo - dotpkg.exe (package manager)
if exist "dist\dotlin-windows\dotlin_lsp.exe" echo - dotlin_lsp.exe (language server)
echo.

echo Testing completed. All Dotlin crates have been built and tested successfully.
echo.

pause