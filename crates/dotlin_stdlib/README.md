# Dotlin Standard Library (stdlib)

The standard library for the Dotlin programming language, providing essential data structures, I/O operations, and utility functions.

## Modules

### Collections
- **Array**: Dynamic arrays with generic type support
- **Map**: Hash map implementation for key-value storage
- **Set**: Hash set for unique value storage

### I/O
- **File**: File system operations (read, write, append)
- **Console**: Console input/output operations
- **Path**: Path manipulation utilities

### Math
- **Functions**: Common mathematical functions (abs, sqrt, pow, etc.)
- **Constants**: Mathematical constants (PI, E, etc.)

### Error Handling
- **Result**: Result type for error handling
- **Option**: Optional value type

### String
- **Extensions**: Additional string manipulation methods

## Usage

The standard library is automatically available in all Dotlin programs. Import specific modules as needed:

```dotlin
import std.collections.Array
import std.io.File
import std.math

fun main() {
    var numbers = Array.new<Int>()
    numbers.push(1)
    numbers.push(2)
    
    var content = File.readFile("data.txt")
    println(content)
    
    var result = math.sqrt(16.0)
    println(result)
}
```

## Development Status

- [ ] Collections module
- [ ] I/O module
- [ ] Math module
- [ ] Error handling types
- [ ] String extensions

## Contributing

See the main Dotlin repository for contribution guidelines.
