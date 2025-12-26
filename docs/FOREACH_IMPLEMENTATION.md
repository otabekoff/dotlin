# ForEach Loop Implementation in Dotlin

## Syntax

The ForEach loop in Dotlin follows the Kotlin syntax:

```dotlin
for (variable in iterable) {
    // loop body
}
```

## Supported Iterables

### Arrays
```dotlin
var numbers = [1, 2, 3, 4, 5]
for (num in numbers) {
    println(num)
}
```

### HashMap Keys
```dotlin
var map = {"key1": 100, "key2": 200, "key3": 300}
for (key in map.keys()) {
    var value = map[key]
    println(key + ": " + value.toString())
}
```

## Implementation Components

### 1. Lexer
- Added `For` token for the `for` keyword
- Added `In` token for the `in` keyword

### 2. AST (Abstract Syntax Tree)
- Added `ForEach` variant to the `Statement` enum:
  ```rust
  ForEach { variable: String, iterable: Expression, body: Box<Statement> }
  ```

### 3. Parser
- Added parsing logic for ForEach statements in the `check_statement` function
- Handles the syntax: `for (variable in iterable) { body }`

### 4. Type Checker
- Added type checking logic for ForEach statements
- Validates that the iterable is of a supported type (Array or HashMap)
- Creates proper scoping for the loop variable

### 5. Code Generator
- Added code generation for ForEach statements
- Generates appropriate Cranelift IR for both array and HashMap iteration
- Handles array iteration by index and HashMap iteration via keys

## How It Works

### Array Iteration
1. Get the array length
2. Initialize a loop index to 0
3. While index < length:
   - Get element at current index
   - Create loop variable with that element
   - Execute loop body
   - Increment index

### HashMap Iteration
1. Get the keys array from the HashMap
2. Get the keys array length
3. Initialize a loop index to 0
4. While index < length:
   - Get key at current index from keys array
   - Get value for that key from the original HashMap
   - Create loop variable with the key
   - Execute loop body
   - Increment index

## Examples

### Basic Array Iteration
```dotlin
fun main() {
    var numbers = [10, 20, 30, 40, 50]
    for (num in numbers) {
        println(num)
    }
}
```

### HashMap Key Iteration
```dotlin
fun main() {
    var data = {"name": "Dotlin", "version": 1, "active": true}
    for (key in data.keys()) {
        var value = data[key]
        println(key + ": " + value.toString())
    }
}
```

## Implementation Status

✅ **Completed Components:**
- Lexer: `for` and `in` tokens
- Parser: ForEach statement parsing
- AST: ForEach statement representation
- Type Checker: ForEach type checking
- Code Generator: ForEach code generation
- Interpreter: ForEach execution support (pending verification)

✅ **Tested With:**
- Array iteration
- HashMap key iteration
- Nested ForEach loops
- Variable scoping within loops