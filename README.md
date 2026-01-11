# Dotlin: A Kotlin-Inspired Native Language

## Overview

**Dotlin** is an experimental, modern, statically typed programming language that reimagines Kotlin as a purely native language, built with Rust. It combines Kotlin's expressive syntax and safety features with true ahead-of-time (AOT) compilation to native machine code, eliminating the JVM dependency entirely.

**File Extension:** `.lin`
**Encoding type**: `Unicode`

---

## Core Philosophy

Dotlin is designed around three foundational principles:

1. **Safety** - Comprehensive compile-time guarantees and null safety by default
2. **Expressiveness** - Modern language features with clean, readable syntax
3. **Interoperability** - Seamless integration with existing native ecosystems

---

## Language Characteristics

### Type System

**Statically Typed with Strong Type Safety**

Dotlin enforces strict type checking at compile time, catching errors before runtime:

```kotlin
val x: Int = "hello" // ❌ Compile-time error
val y: String = 42   // ❌ Compile-time error
```

**Type Inference**

While types are inferred automatically, this doesn't compromise static typing:

```kotlin
val x = 10              // Inferred as Int
val name = "Dotlin"     // Inferred as String
val items = listOf(1, 2) // Inferred as List<Int>
```

✔ Still statically typed  
✔ Still checked at compile time  
✔ No runtime type ambiguity

**Strong Typing**

No implicit type coercions or automatic conversions:

```kotlin
val count = 10
val message = count.toString() // ✅ Explicit conversion required
val result = "Value: " + count // ❌ No automatic Int → String coercion
```

### Compilation Model

**Pure Ahead-of-Time (AOT) Compilation**

```
Dotlin source (.lin)
        ↓
    Rust-based compiler
        ↓
    LLVM IR (optional intermediate)
        ↓
Native machine code (executable)
```

**Key Advantages:**
- ✅ No JVM dependency
- ✅ No Just-In-Time (JIT) compilation overhead
- ✅ Faster startup times
- ✅ Predictable performance characteristics
- ✅ Smaller binary footprint
- ✅ True native execution

---

## Abstraction Level

**High-Level with Low-Level Control**

Dotlin is fundamentally a high-level language but provides controlled access to lower-level operations when needed:

| Aspect                    | Level | Notes                                           |
| ------------------------- | ----- | ----------------------------------------------- |
| Default programming model | High  | Rich abstractions, automatic memory management  |
| Performance-critical code | Mid   | Optional manual optimizations                   |
| Unsafe operations         | Low   | Explicit `unsafe` blocks where necessary        |
| Memory control            | Mid   | ARC-like reference counting with escape hatches |

**High-Level Features:**
- Automatic memory management
- Rich standard library
- Advanced abstractions (lambdas, coroutines, DSLs)
- Declarative programming support

**Lower-Level Access:**
- Manual memory layout control (when needed)
- Inline assembly (explicit)
- FFI (Foreign Function Interface) for C/C++
- Zero-cost abstractions where possible

---

## Memory Management

**Automatic Reference Counting (ARC) + Smart Optimizations**

Dotlin uses a hybrid memory management approach:

- **Primary:** Automatic Reference Counting (ARC)
  - No manual `malloc`/`free`
  - Deterministic deallocation
  - Predictable performance
  
- **Optimizations:**
  - Compile-time escape analysis
  - Stack allocation for short-lived objects
  - Optional arena allocators for performance-critical sections

- **Safety:**
  - No direct pointer manipulation (by default)
  - Bounds checking on collections
  - Optional `unsafe` blocks for advanced use cases

**Memory Safety Guarantees:**

```kotlin
val data = listOf(1, 2, 3)
val item = data[10] // ❌ Runtime bounds check, prevents buffer overflow

// In performance-critical code with proven safety:
unsafe {
    val item = data.getUnchecked(2) // ⚠️ No bounds check
}
```

---

## Paradigm Support

Dotlin is truly multi-paradigm, supporting multiple programming styles:

| Paradigm            | Support | Examples                                               |
| ------------------- | ------- | ------------------------------------------------------ |
| **Object-Oriented** | ✅ Full  | Classes, inheritance, interfaces, polymorphism         |
| **Functional**      | ✅ Full  | Higher-order functions, immutability, pattern matching |
| **Imperative**      | ✅ Full  | Loops, mutable state, procedural code                  |
| **Declarative**     | ✅ Full  | DSLs, builders, declarative APIs                       |
| **Concurrent**      | ✅ Full  | Coroutines, async/await, structured concurrency        |
| **Generic**         | ✅ Full  | Type parameters, constraints, variance                 |

**Example - Multiple Paradigms:**

```kotlin
// Object-Oriented
class User(val name: String, val age: Int) {
    fun greet() = "Hello, I'm $name"
}

// Functional
val adults = users.filter { it.age >= 18 }
                  .map { it.name }
                  .sorted()

// Declarative DSL
html {
    body {
        h1 { +"Welcome to Dotlin" }
        p { +"A native Kotlin alternative" }
    }
}

// Concurrent
suspend fun fetchData(): Data {
    val result1 = async { apiCall1() }
    val result2 = async { apiCall2() }
    return combine(result1.await(), result2.await())
}
```

---

## Safety Features

Dotlin prioritizes safety over convenience, providing compile-time guarantees that prevent entire classes of bugs:

| Feature                        | Purpose                           | Example                               |
| ------------------------------ | --------------------------------- | ------------------------------------- |
| **Null Safety**                | Eliminate null pointer exceptions | `String` vs `String?`                 |
| **Immutability by Default**    | Safer state management            | `val` (immutable) vs `var` (mutable)  |
| **Smart Casts**                | Reduce runtime checks             | Automatic type narrowing after checks |
| **Exhaustive When**            | Ensure all cases handled          | Compiler error on missing branches    |
| **Sealed Classes**             | Controlled type hierarchies       | Finite, known subtype sets            |
| **Bounds Checking**            | Prevent buffer overflows          | Array/collection access validation    |
| **No Uninitialized Variables** | All variables must be initialized | Compile-time enforcement              |
| **Visibility Modifiers**       | Encapsulation enforcement         | `private`, `internal`, `public`       |

**Null Safety Example:**

```kotlin
// Non-nullable by default
val name: String = "Dotlin"
name = null // ❌ Compile error

// Nullable types are explicit
val optionalName: String? = null
optionalName.length // ❌ Compile error

// Safe access
val length = optionalName?.length // ✅ Returns Int?
val length = optionalName?.length ?: 0 // ✅ Provide default
```

**Smart Casts:**

```kotlin
fun process(value: Any) {
    if (value is String) {
        println(value.length) // ✅ Automatically cast to String
    }
}
```

**Exhaustive When:**

```kotlin
sealed class Result {
    data class Success(val data: String) : Result()
    data class Error(val message: String) : Result()
}

fun handle(result: Result) = when (result) {
    is Result.Success -> println(result.data)
    is Result.Error -> println(result.message)
    // ✅ Compiler ensures all cases covered
}
```

---

## Runtime & Dependencies

**No Runtime Required**

- ✅ Pure native executables
- ✅ No interpreter needed
- ✅ No virtual machine dependency
- ✅ Minimal standard library embedded in binary
- ✅ Static linking by default (dynamic linking optional)

**Binary Characteristics:**
- Self-contained executables
- Platform-specific compilation
- Optional minimal runtime for coroutines/async (statically linked)

---

## Interoperability

Dotlin is designed for seamless integration with existing ecosystems:

### Native Interop (C/C++)

**Priority: High**

```kotlin
// Call C functions directly
@CInterop
external fun printf(format: CString, vararg args: Any): Int

// Use C libraries
@CLibrary("sqlite3")
external fun sqlite3_open(filename: CString, db: CPointer<SQLite3>): Int
```

### Kotlin Interop

**Priority: Medium**

- Call Kotlin/JVM libraries (through JNI bridge)
- Share data structures and APIs
- Gradual migration path from Kotlin

### Python Interop

**Priority: Medium**

```kotlin
// Embed Python (via libpython)
@PythonInterop
fun callPython(code: String): PyObject
```

### JavaScript/TypeScript Interop

**Priority: Low-Medium**

- Embed JavaScript engine (QuickJS/V8)
- Useful for scripting and plugins

### Rust Interop

**Priority: High** (native FFI)

- Since Dotlin compiler is built with Rust
- Direct Rust library integration
- Share memory models and safety guarantees

---

## Platform Targets

Dotlin compiles to native code for multiple platforms:

| Platform                          | Support   | Notes              |
| --------------------------------- | --------- | ------------------ |
| **Linux** (x86_64, ARM64)         | ✅ Primary | Full support       |
| **macOS** (x86_64, Apple Silicon) | ✅ Primary | Full support       |
| **Windows** (x86_64)              | ✅ Primary | MSVC and MinGW     |
| **BSD**                           | ⭐ Planned | FreeBSD, OpenBSD   |
| **WebAssembly**                   | ⭐ Planned | Browser and WASI   |
| **iOS/Android**                   | ⭐ Future  | Native mobile apps |
| **Embedded**                      | ⭐ Future  | Bare metal, RTOS   |

---

## Use Cases & Applications

| Domain                   | Fit   | Notes                                             |
| ------------------------ | ----- | ------------------------------------------------- |
| **Systems Programming**  | ⭐⭐⭐⭐⭐ | Operating systems, drivers, low-level tools       |
| **Backend Services**     | ⭐⭐⭐⭐⭐ | High-performance servers, microservices           |
| **CLI Tools**            | ⭐⭐⭐⭐⭐ | Fast startup, single binary distribution          |
| **Embedded Systems**     | ⭐⭐⭐⭐  | IoT, robotics, real-time systems                  |
| **Game Development**     | ⭐⭐⭐⭐  | Game engines, performance-critical code           |
| **Desktop Applications** | ⭐⭐⭐⭐  | Native GUI apps with small footprint              |
| **Data Processing**      | ⭐⭐⭐⭐  | ETL pipelines, data transformation                |
| **Compilers & Tools**    | ⭐⭐⭐⭐  | Language tooling, build systems                   |
| **Networking**           | ⭐⭐⭐⭐  | Proxies, load balancers, protocol implementations |
| **WebAssembly**          | ⭐⭐⭐   | Browser and edge computing                        |
| **Android Development**  | ⭐⭐    | Possible via NDK (not primary use case)           |
| **Scientific Computing** | ⭐⭐⭐   | Numerical computation, simulations                |

---

## Language Features

### Modern Syntax

```kotlin
// Data classes
data class Person(val name: String, val age: Int)

// Extension functions
fun String.reversed() = this.reversed()

// Operator overloading
data class Vector(val x: Int, val y: Int) {
    operator fun plus(other: Vector) = Vector(x + other.x, y + other.y)
}

// Destructuring
val (name, age) = Person("Alice", 30)

// String templates
val greeting = "Hello, $name! You are $age years old."

// Ranges
for (i in 1..10) { println(i) }

// Named arguments
fun createUser(name: String, age: Int = 0, email: String? = null) { }
createUser(name = "Bob", email = "bob@example.com")
```

### Coroutines & Concurrency

```kotlin
// Suspend functions
suspend fun fetchUser(id: Int): User {
    delay(100) // Non-blocking delay
    return database.getUser(id)
}

// Structured concurrency
coroutineScope {
    val users = async { fetchUsers() }
    val posts = async { fetchPosts() }
    combine(users.await(), posts.await())
}

// Channels
val channel = Channel<Int>()
launch {
    for (i in 1..5) channel.send(i)
    channel.close()
}
```

### Pattern Matching

```kotlin
when (value) {
    is Int -> println("Integer: $value")
    is String -> println("String: $value")
    in 1..10 -> println("In range")
    !in validValues -> println("Invalid")
    else -> println("Unknown")
}
```

### Type Classes / Traits

```kotlin
interface Printable {
    fun print()
}

// Extension to existing types
fun Int.print() = println(this)

// Contextual receivers
context(Logger)
fun performOperation() {
    log("Starting operation")
}
```

---

## Standard Library

Dotlin provides a comprehensive standard library organized into modules:

### Core Modules

- **`dotlin.core`** - Fundamental types, collections, ranges
- **`dotlin.io`** - File I/O, streams, readers/writers
- **`dotlin.concurrent`** - Coroutines, channels, async primitives
- **`dotlin.collections`** - Advanced data structures
- **`dotlin.text`** - String processing, regex, encoding
- **`dotlin.math`** - Mathematical operations, numerics
- **`dotlin.time`** - Date/time handling
- **`dotlin.net`** - Networking, HTTP, sockets
- **`dotlin.system`** - OS interaction, process management
- **`dotlin.unsafe`** - Low-level operations, raw pointers

### Platform-Specific Modules

- **`dotlin.platform.linux`**
- **`dotlin.platform.macos`**
- **`dotlin.platform.windows`**

---

## Tooling Ecosystem

### Compiler

- **`dotlinc`** - Main compiler (written in Rust)
  - Multi-stage compilation
  - Incremental compilation support
  - Rich error messages with suggestions
  - LSP (Language Server Protocol) integration

### Build System

- **`dotlin build`** - Native build tool
  - Dependency management
  - Multi-platform builds
  - Caching and incremental builds
  - Package registry integration

### Package Manager

- **Dotlin Registry** - Central package repository
  - Semantic versioning
  - Dependency resolution
  - Private registry support

### Development Tools

- **IDE Support:**
  - IntelliJ IDEA plugin
  - VS Code extension
  - Vim/Neovim LSP support
  
- **Debugging:**
  - GDB/LLDB integration
  - Native debugger support
  - Stack trace preservation

- **Testing:**
  - Built-in test framework
  - Property-based testing
  - Mocking and stubbing support

- **Documentation:**
  - KDoc-style documentation comments
  - Automated doc generation
  - Examples in documentation

---

## Performance Characteristics

| Metric               | Dotlin       | Kotlin/JVM | Notes                                   |
| -------------------- | ------------ | ---------- | --------------------------------------- |
| **Startup Time**     | < 1ms        | ~100-500ms | Native vs JVM warmup                    |
| **Memory Footprint** | Low          | High       | No JVM overhead                         |
| **Peak Performance** | High         | Very High  | After JIT warmup, JVM can optimize more |
| **Predictability**   | Excellent    | Good       | No GC pauses                            |
| **Binary Size**      | Small-Medium | Large      | No runtime bundling needed              |
| **Compile Time**     | Fast-Medium  | Fast       | AOT vs JIT tradeoff                     |

---

## Example Programs

### Hello World

```kotlin
// hello.lin
fun main() {
    println("Hello, Dotlin!")
}
```

Compile and run:
```bash
dotlinc hello.lin -o hello
./hello
```

### HTTP Server

```kotlin
// server.lin
import dotlin.net.http.*

suspend fun main() {
    val server = HttpServer(port = 8080) {
        get("/") { request ->
            Response.ok("Welcome to Dotlin!")
        }
        
        get("/users/:id") { request ->
            val userId = request.params["id"]
            Response.json(mapOf("id" to userId))
        }
    }
    
    println("Server running on http://localhost:8080")
    server.start()
}
```

### Concurrent Data Processing

```kotlin
// pipeline.lin
import dotlin.concurrent.*

data class Record(val id: Int, val value: String)

suspend fun processPipeline(input: List<Record>): List<Record> = coroutineScope {
    val channel = Channel<Record>(capacity = 100)
    
    // Producer
    launch {
        input.forEach { channel.send(it) }
        channel.close()
    }
    
    // Workers
    val results = (1..4).map { workerId ->
        async {
            buildList {
                for (record in channel) {
                    add(processRecord(record, workerId))
                }
            }
        }
    }
    
    results.flatMap { it.await() }
}

fun processRecord(record: Record, workerId: Int): Record {
    // Expensive processing
    return record.copy(value = "${record.value}_processed_by_$workerId")
}
```

---

## Comparison Summary

### Dotlin vs Kotlin/JVM

| Aspect       | Dotlin               | Kotlin/JVM               |
| ------------ | -------------------- | ------------------------ |
| Runtime      | None (native)        | JVM required             |
| Startup      | Instant              | Slow (JVM warmup)        |
| Memory       | Lower baseline       | Higher (JVM overhead)    |
| Distribution | Single binary        | JAR + JVM                |
| Platform     | Native each platform | Write once, run anywhere |
| Ecosystem    | Growing              | Mature                   |
| Interop      | C/C++, Rust          | Java, JVM languages      |

### Dotlin vs Rust

| Aspect        | Dotlin                 | Rust                    |
| ------------- | ---------------------- | ----------------------- |
| Syntax        | Familiar (Kotlin-like) | Steeper learning curve  |
| Memory Safety | ARC + compiler checks  | Borrow checker          |
| Null Safety   | Built-in (`?` types)   | `Option<T>`             |
| GC            | Automatic (ARC)        | No GC, manual ownership |
| Abstraction   | Higher-level           | More explicit           |
| Concurrency   | Coroutines (easier)    | Async/await + Send/Sync |

### Dotlin vs Go

| Aspect         | Dotlin                      | Go                     |
| -------------- | --------------------------- | ---------------------- |
| Type System    | Richer (generics, variance) | Simpler                |
| Error Handling | Exceptions + Result types   | Explicit error returns |
| Concurrency    | Coroutines                  | Goroutines             |
| Memory         | ARC                         | GC with STW pauses     |
| Performance    | Similar                     | Similar                |

---

## Project Status & Roadmap

### Current Status: **Experimental / Early Development**

### Phase 1: Foundation (Current)
- ✅ Language specification
- 🚧 Core compiler (Rust-based)
- 🚧 Basic type system
- 🚧 Memory management (ARC)
- 📅 Standard library essentials

### Phase 2: Core Features
- 📅 Null safety implementation
- 📅 Coroutines runtime
- 📅 FFI (C interop)
- 📅 Basic tooling (LSP, formatter)

### Phase 3: Ecosystem
- 📅 Package manager
- 📅 Standard library expansion
- 📅 IDE plugins
- 📅 Documentation

### Phase 4: Production Ready
- 📅 Stability guarantees
- 📅 Performance optimizations
- 📅 Cross-platform support
- 📅 Community growth

---

## Contributing & Community

**Project Philosophy:**
- Open source (Apache 2.0 / MIT dual license)
- Community-driven development
- Transparent decision-making
- Welcoming to contributors

**Get Involved:**
- GitHub: `github.com/dotlin-lang/dotlin`
- Discord: Community chat
- Forum: Technical discussions
- RFC Process: Language proposals

---

## Formal Definition

**Dotlin** is a statically and strongly typed, high-level, compiled, multi-paradigm programming language that targets native platforms through ahead-of-time compilation. Built with Rust, it emphasizes memory safety through automatic reference counting, null safety by default, and modern concurrency primitives. Dotlin provides rich abstractions including classes, higher-order functions, coroutines, and domain-specific language support, while maintaining interoperability with C, C++, Rust, and other native ecosystems. The language is designed for systems programming, high-performance backend services, and applications requiring predictable performance characteristics without runtime dependencies.

---

## Quick Reference

```kotlin
// File: example.lin

// Nullable types
val name: String = "Dotlin"
val optional: String? = null

// Collections
val numbers = listOf(1, 2, 3, 4, 5)
val filtered = numbers.filter { it > 2 }

// Data classes
data class User(val id: Int, val name: String)

// Sealed classes
sealed class Result<out T> {
    data class Success<T>(val data: T) : Result<T>()
    data class Error(val message: String) : Result<Nothing>()
}

// Extension functions
fun String.shout() = this.uppercase() + "!"

// Coroutines
suspend fun fetchData(): Data {
    delay(100)
    return Data()
}

// Pattern matching
fun handle(result: Result<String>) = when (result) {
    is Result.Success -> println(result.data)
    is Result.Error -> eprintln(result.message)
}

// Main entry point
fun main() {
    println("Welcome to Dotlin!")
}
```

---

**Dotlin: Native performance, Kotlin elegance** ⚡