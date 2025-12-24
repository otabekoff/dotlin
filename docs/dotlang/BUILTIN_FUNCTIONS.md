# Built-in Functions in Dotlin

This document describes the built-in functions available in the Dotlin programming language.

## File I/O Functions

### `readFile(path: String): String`
Reads the entire content of a file and returns it as a string.

**Example:**
```dotlin
fun main() {
    var content = readFile("input.txt")
    println(content)
}
```

### `writeFile(path: String, content: String): Int`
Writes content to a file, overwriting any existing content. Returns 0 on success, -1 on failure.

**Example:**
```dotlin
fun main() {
    var result = writeFile("output.txt", "Hello, World!")
    if (result == 0) {
        println("File written successfully")
    } else {
        println("File write failed")
    }
}
```

### `appendFile(path: String, content: String): Int`
Appends content to a file. Returns 0 on success, -1 on failure.

**Example:**
```dotlin
fun main() {
    var result = appendFile("log.txt", "New log entry\n")
    if (result == 0) {
        println("Content appended successfully")
    } else {
        println("Append failed")
    }
}
```

### `fileExists(path: String): Boolean`
Checks if a file exists at the given path.

**Example:**
```dotlin
fun main() {
    if (fileExists("data.txt")) {
        var content = readFile("data.txt")
        println("File content: " + content)
    } else {
        println("File does not exist")
    }
}
```

## Console I/O Functions

### `print(value: String)`
Prints a value to the console without adding a newline character.

**Example:**
```dotlin
fun main() {
    print("Enter your name: ")
    var name = readLine()
    print("Hello, " + name + "!")
}
```

### `readLine(): String`
Reads a line of input from the user. Returns the input as a string.

**Example:**
```dotlin
fun main() {
    print("What is your name? ")
    var name = readLine()
    println("Hello, " + name + "!")
}
```

## Standard Output Functions

### `println(value)`
Prints a value to the console and adds a newline character.

**Example:**
```dotlin
fun main() {
    println("Hello, World!")
    var x = 42
    println(x)
}
```

## String Operations

### String Concatenation
Strings can be concatenated using the `+` operator.

**Example:**
```dotlin
fun main() {
    var greeting = "Hello, " + "Dotlin!"
    println(greeting)  // Output: Hello, Dotlin!
}
```

### String Length
The length of a string can be accessed using the `.length` property.

**Example:**
```dotlin
fun main() {
    var text = "Hello"
    var len = text.length
    println(len)  // Output: 5
}
```