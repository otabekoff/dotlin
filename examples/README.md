# Dotlin Examples

This directory contains example programs demonstrating various features of the Dotlin programming language.

## Directory Structure

- **[basic/](basic/)** - Simple examples for beginners
  - Hello World
  - Variables and types
  - Functions
  - Control flow
  - String operations

- **[intermediate/](intermediate/)** - More complex examples
  - File I/O operations
  - Data structures
  - Error handling
  - Algorithms

- **[advanced/](advanced/)** - Advanced use cases
  - Complex applications
  - Performance optimization
  - Advanced patterns

## Running Examples

### Compile and Run

```bash
# Compile an example
dotc examples/basic/hello.lin -o hello

# Run the executable
./hello  # Unix/Linux/macOS
hello.exe  # Windows
```

### Using the REPL

```bash
# Start the REPL
dotrepl

# Load and run an example
>> // Copy and paste code from examples
```

## Basic Examples

### Hello World
```dotlin
fun main() {
    println("Hello, World!")
}
```

### Variables and Types
```dotlin
fun main() {
    var name: String = "Dotlin"
    var version: Float = 0.1
    var count: Int = 42
    var isAwesome: Boolean = true
    
    println(name)
    println(version)
    println(count)
    println(isAwesome)
}
```

### Functions
```dotlin
fun add(a: Int, b: Int): Int {
    return a + b
}

fun greet(name: String): String {
    return "Hello, " + name + "!"
}

fun main() {
    var sum = add(10, 20)
    println(sum)
    
    var greeting = greet("World")
    println(greeting)
}
```

### String Operations
```dotlin
fun main() {
    var s1 = "Hello"
    var s2 = "World"
    
    // Concatenation
    var greeting = s1 + ", " + s2 + "!"
    println(greeting)
    
    // Length
    println(greeting.length)
    
    // Comparison
    if (s1 < s2) {
        println("s1 comes before s2")
    }
}
```

## Contributing Examples

We welcome contributions of new examples! Please:

1. Ensure your example compiles and runs correctly
2. Add comments explaining the code
3. Include a description in this README
4. Place it in the appropriate difficulty level directory
5. Follow the Dotlin style guide

See [CONTRIBUTING.md](../CONTRIBUTING.md) for more details.

## License

All examples are dual-licensed under MIT and Apache 2.0, same as the Dotlin project.
