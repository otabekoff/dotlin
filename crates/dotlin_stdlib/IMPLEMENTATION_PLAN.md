# Dotlin Standard Library Implementation Plan

## Phase 4.1: Core Standard Library (v0.2.0)

### Priority 1: Array Support (Week 1-2)

#### AST Changes
- [ ] Add `Type::Array(Box<Type>)` variant
- [ ] Add array literal syntax: `[1, 2, 3]`
- [ ] Add indexing expression: `arr[index]`
- [ ] Add array type annotations: `var arr: [Int]`

#### Parser Changes
- [ ] Parse array literals: `[expr, expr, ...]`
- [ ] Parse index expressions: `expr[expr]`
- [ ] Parse array type syntax

#### Type Checker Changes
- [ ] Validate array element types
- [ ] Check index bounds at compile time (where possible)
- [ ] Infer array types from literals

#### Codegen Changes
- [ ] Array allocation
- [ ] Element access
- [ ] Bounds checking
- [ ] Array methods (push, pop, length)

#### Runtime Support
- [ ] `dotlin_array_new(element_size, capacity)`
- [ ] `dotlin_array_push(array, element)`
- [ ] `dotlin_array_pop(array)`
- [ ] `dotlin_array_get(array, index)`
- [ ] `dotlin_array_set(array, index, value)`
- [ ] `dotlin_array_length(array)`

### Priority 2: File I/O (Week 2-3)

#### Runtime Functions
- [ ] `dotlin_file_read(path: *const u8) -> *const u8`
- [ ] `dotlin_file_write(path: *const u8, content: *const u8) -> i64`
- [ ] `dotlin_file_append(path: *const u8, content: *const u8) -> i64`
- [ ] `dotlin_file_exists(path: *const u8) -> i8`

#### Built-in Functions
- [ ] `readFile(path: String): String`
- [ ] `writeFile(path: String, content: String): Int`
- [ ] `appendFile(path: String, content: String): Int`
- [ ] `fileExists(path: String): Boolean`

### Priority 3: Enhanced Console I/O (Week 3)

#### Runtime Functions
- [ ] `dotlin_read_line() -> *const u8`
- [ ] `dotlin_print(value: *const u8)` (no newline)

#### Built-in Functions
- [ ] `print(value)` - no newline
- [ ] `readLine(): String`
- [ ] `readInt(): Int` (with error handling)

### Priority 4: Math Module (Week 4)

#### Runtime Functions
- [ ] `dotlin_math_abs_i64(x: i64) -> i64`
- [ ] `dotlin_math_abs_f64(x: f64) -> f64`
- [ ] `dotlin_math_min_i64(a: i64, b: i64) -> i64`
- [ ] `dotlin_math_max_i64(a: i64, b: i64) -> i64`
- [ ] `dotlin_math_pow(base: f64, exp: f64) -> f64`
- [ ] `dotlin_math_sqrt(x: f64) -> f64`
- [ ] `dotlin_math_floor(x: f64) -> f64`
- [ ] `dotlin_math_ceil(x: f64) -> f64`
- [ ] `dotlin_math_round(x: f64) -> f64`

#### Constants
- [ ] `PI = 3.141592653589793`
- [ ] `E = 2.718281828459045`

### Priority 5: Result/Option Types (Week 5-6)

#### AST Changes
- [ ] Add enum support
- [ ] Add pattern matching (basic)
- [ ] Add generic type parameters

#### Type System
- [ ] `Result<T, E>` enum with `Ok(T)` and `Err(E)`
- [ ] `Option<T>` enum with `Some(T)` and `None`

#### Methods
- [ ] `isOk()`, `isErr()`, `unwrap()`, `unwrapOr(default)`
- [ ] `isSome()`, `isNone()`, `unwrap()`, `unwrapOr(default)`

## Implementation Order

1. **Week 1-2**: Array support (most requested feature)
2. **Week 2-3**: File I/O (essential for practical programs)
3. **Week 3**: Enhanced console I/O
4. **Week 4**: Math module
5. **Week 5-6**: Result/Option types (requires enum support)

## Testing Strategy

For each feature:
1. Write unit tests in Rust for runtime functions
2. Create integration tests in Dotlin
3. Add to CI/CD pipeline
4. Document with examples

## Example Programs to Support

### Array Example
```dotlin
fun main() {
    var numbers = [1, 2, 3, 4, 5]
    numbers.push(6)
    println(numbers.length)  // 6
    println(numbers[0])      // 1
}
```

### File I/O Example
```dotlin
fun main() {
    var content = readFile("input.txt")
    println(content)
    
    writeFile("output.txt", "Hello, World!")
}
```

### Math Example
```dotlin
import math

fun main() {
    var x = math.sqrt(16.0)
    println(x)  // 4.0
    
    var y = math.pow(2.0, 8.0)
    println(y)  // 256.0
}
```

### Result Example
```dotlin
fun divide(a: Int, b: Int): Result<Int, String> {
    if (b == 0) {
        return Err("Division by zero")
    }
    return Ok(a / b)
}

fun main() {
    var result = divide(10, 2)
    if (result.isOk()) {
        println(result.unwrap())
    }
}
```
