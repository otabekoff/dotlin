# Dotlin Language - Missing Kotlin Features

This document outlines the features that are currently missing from Dotlin to achieve full Kotlin syntax compatibility.

## Currently Implemented Features ✅

### Basic Syntax
- Variable declarations (`var` keyword)
- Function declarations (`fun` keyword)
- Block statements
- Return statements
- If/else statements
- While loops
- Comments

### Data Types
- Integer literals
- Float literals
- String literals
- Boolean literals
- Character literals
- Array literals (`[1, 2, 3]`)
- HashMap literals (`{"key": value}`)

### Operations
- Arithmetic operators (`+`, `-`, `*`, `/`)
- Comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`)
- Logical operators (`&&`, `||`, `!`)
- Assignment operators (`=`, `+=`, `-=`, `*=`, `/=`)
- Increment/decrement operators (`++`, `--`)
- Indexing operations (`array[index]`, `map[key]`)

### Methods and Functions
- Function calls
- Method calls on objects
- Member access (`object.method()`)
- Type conversion methods (`toInt()`, `toFloat()`, `toString()`)

### Collections
- Array push/pop methods
- HashMap iteration methods (`keys()`, `values()`, `size()`, `entries()`)

## Missing Kotlin Features 🚫

### Language Constructs
- **`val` keyword** - Immutable variable declarations (currently only `var`)
- **`const` keyword** - Compile-time constants
- **`let` scope function** - Scope control functions
- **`apply`, `also`, `run`, `with` functions** - Scope functions
- **`when` expressions** - Switch-like expressions
- **`try`, `catch`, `finally`** - Exception handling
- **`throw` expressions** - Exception throwing
- **`is` operator** - Type checking
- **`as` operator** - Type casting
- **`in` operator** - Range and collection membership

### Control Flow
- **For loops** - `for (item in collection)` syntax
- **For-each loops** - Enhanced for loops
- **Do-while loops** - Post-test loops
- **Break and continue** - Loop control statements
- **Labels** - Loop and expression labels

### Functions
- **Lambda expressions** - Anonymous functions
- **Higher-order functions** - Functions that take other functions as parameters
- **Function references** - `::functionName` syntax
- **Default parameter values** - Parameters with default values
- **Named arguments** - Calling functions with named parameters
- **Varargs** - Variable argument functions
- **Extension functions** - Adding methods to existing types
- **Infix functions** - Infix notation functions
- **Inline functions** - `inline` keyword support
- **Tailrec functions** - Tail recursive functions

### Classes and Object-Oriented Features
- **Class declarations** - `class` keyword and syntax
- **Properties** - Class member variables
- **Methods** - Class member functions
- **Constructors** - Primary and secondary constructors
- **Inheritance** - `open`, `override` keywords
- **Interfaces** - `interface` keyword
- **Abstract classes** - `abstract` keyword
- **Data classes** - `data class` syntax
- **Enum classes** - `enum class` syntax
- **Sealed classes** - `sealed class` syntax
- **Object declarations** - `object` keyword
- **Companion objects** - `companion object` syntax
- **Delegation** - `by` keyword

### Collections and Ranges
- **List interface** - Standard list operations
- **Set interface** - Standard set operations
- **Map interface** - Standard map operations
- **Range operators** - `..`, `rangeTo`, `downTo`, `step`
- **Collection operations** - `map`, `filter`, `forEach`, etc.
- **Mutable vs immutable collections** - Different collection types

### Null Safety
- **Nullable types** - `Type?` syntax
- **Safe calls** - `?.` operator
- **Elvis operator** - `?:` operator
- **Not-null assertions** - `!!` operator
- **Platform types** - Interop with non-null-safe systems

### Generics
- **Generic types** - `List<T>`, `Map<K, V>` syntax
- **Generic functions** - Functions with type parameters
- **Type variance** - `in`, `out` keywords
- **Reified generics** - `reified` keyword

### Coroutines
- **Coroutine builders** - `launch`, `async`, etc.
- **Suspending functions** - `suspend` keyword
- **Coroutine scopes** - Scope management

### Advanced Features
- **Annotations** - `@Annotation` syntax
- **Visibility modifiers** - `public`, `private`, `protected`, `internal`
- **Visibility modifiers** - `public`, `private`, `protected`, `internal`
- **Imports** - `import` statements
- **Packages** - `package` declarations
- **Destructuring declarations** - `val (a, b) = pair` syntax
- **String templates** - `${expression}` syntax
- **Multi-line strings** - Triple-quoted strings
- **Raw strings** - Unescaped string literals
- **String interpolation** - Direct variable inclusion in strings

### Standard Library
- **Standard library functions** - `println`, `print`, `readLine`, etc.
- **Collection utilities** - All collection operations
- **I/O operations** - File, console I/O
- **Math functions** - Mathematical utilities
- **Date/Time API** - Date and time handling
- **Regex support** - Regular expressions
- **JSON/XML processing** - Data format handling

### Concurrency
- **Threading** - Thread creation and management
- **Synchronization** - Locks, mutexes
- **Atomic operations** - Thread-safe operations

### Platform-Specific Features
- **JVM interop** - If targeting JVM
- **JavaScript interop** - If targeting JS
- **Native interop** - C interop for native targets

## Implementation Priority

### High Priority
1. `val` keyword for immutable variables
2. Exception handling (`try`, `catch`, `finally`, `throw`)
3. For loops and enhanced control flow
4. Lambda expressions and higher-order functions
5. Classes and object-oriented features
6. Null safety features

### Medium Priority
1. Collections and ranges
2. Generics
3. Scope functions
4. String templates and multi-line strings
5. Standard library expansion

### Low Priority
1. Coroutines
2. Advanced annotations
3. Platform-specific features
4. Advanced interop features

## Current Limitations

- No immutable variable support (`val`)
- No exception handling
- Limited collection operations
- No null safety
- No object-oriented programming
- No advanced type system features
- No standard library beyond basic operations