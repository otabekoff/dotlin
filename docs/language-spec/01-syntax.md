# Dotlin — Basic Syntax Overview

This section shows the minimal Dotlin equivalents for common Kotlin basics the language should support.

## Package definition and imports

Package specification should be at the top of the source file:

```lin
package my.demo

import dotlin.text.*

// ...
```

It is not required to match directories and packages: source files can be placed arbitrarily in the file system.

## Program entry point

A Dotlin program starts at a `main` function:

```lin
fun main() {
	println("Hello world!")
}
```

`main` may also accept arguments:

```lin
fun main(args: Array<String>) {
	println(args[0])
    println(args.contentToString())
}
```

## Print / standard output

`print` writes  its argument to the standard output without a trailing newline; `println` appends a newline:

```lin
print("Hello ")
print("world!")

println("Hello world!")
println(42)
```

## Read from standard input

The `readln()` function reads from the standard input. This function reads the entire line the user enters as a string.

You can use the `println()`, `readln()`, and `print()` functions together to print messages requesting 
and showing user input:

```lin
// Prints a message to request input
println("Enter any word: ")

// Reads and stores the user input. For example: Happiness
val yourWord = readln()

// Prints a message with the input
print("You entered the word: ")
print(yourWord)
// You entered the word: Happiness
```

## Functions

Normal function with explicit return type. It comes with two `Int` parameters and `Int` return type:

```lin
fun sum(a: Int, b: Int): Int {
	return a + b
}

// Usage of normal function `sum` in `main` function:
fun main() {
    print("sum of 3 and 5 is ")
    println(sum(3, 5))
}
```

A function body can be an expression. Its return type is inferred:

```lin
fun sum(a: Int, b: Int) = a + b

// Usage of expression-bodied function `sum` in `main` function
fun main() {
    println("sum of 19 and 23 is ${sum(19, 23)}")
}
```

Function returning no meaningful value uses `Unit` (omitted most of the time):

```lin
fun printSum(a: Int, b: Int): Unit {
    println("sum of $a and $b is ${a + b}")
}

// Omitted version:
// fun printSum(a: Int, b: Int) {
//     println("sum of $a and $b is ${a + b}")
// }

// Usage in `main` function.
fun main() {
    printSum(-1, 8)
}
```

## Variables

In Dotlin, you declare a variable starting with a keyword, `val` (immutable) or `var` (mutable), followed by the name of the variable.

Use the `val` keyword to declare variables that are assigned a value only once. These are immutable, read-only local variables that can't be reassigned a different value after initialization: 

```lin
fun main() {
    // Declares the variable x and initializes it with the value of 5
    val x: Int = 5
	// Value is 5 here
    println(x) // Prints: 5
	
	var y = 0
	y += 1
}
```

Use the `var` keyword to declare variables that can be reassigned. These are mutable variables, and you can change their values after initialization:

```lin
fun main() {
    // Declares the variable x and initializes it with the value of 5
    var x: Int = 5
    // Reassigns a new value of 6 to the variable x
    x += 1
    // Value is 6 here
    println(x) // Prints: 6
}
```

Dotlin supports type inference and automatically identifies the data type of a declared variable. When declaring a variable, you can omit the type after the variable name:

```lin
fun main() {
    // Declares the variable x with the value of 5;`Int` type is inferred
    val x = 5
    // 5
    println(x)
}
```

As the type inference is supported; a variable declared without an initializer requires an explicit type:

```lin
fun main() {
	val c: Int
	c = 3
}
```

You can use variables only after initializing them. You can either initialize a variable at the moment of declaration or declare a variable first and initialize it later. 
In the second case, you must specify the data type:

```lin
fun main() {
    // Initializes the variable x at the moment of declaration; type is not required
    val x = 5
    // Declares the variable c without initialization; type is required
    val c: Int
    // Initializes the variable c after declaration 
    c = 3
    // Value of x is 5 here
    // Value of c is 3 here
    println(x) // Prints: 5
    println(c) // Prints: 3
}
```

You can declare variables at the top level, meaning the variables before the `main` function are allowed:

```lin
val PI = 3.14
var x = 0

fun incrementX() {
    x += 1
}
// x = 0; PI = 3.14
// incrementX()
// x = 1; PI = 3.14

fun main() {
    println("x = $x; PI = $PI")
    incrementX()
    println("incrementX()")
    println("x = $x; PI = $PI")
}
```

## Classes and instances

Declare classes with `class`:
```lin
class Shape
```

Properties of a class can be listed in its declaration or body: 

```lin
class Rectangle(val height: Double, val length: Double) {
    val perimeter = (height + length) * 2 
}
```

The default constructor with parameters listed in the class declaration is available automatically:

```lin
class Rectangle(val height: Double, val length: Double) {
    val perimeter = (height + length) * 2 
}
fun main() {
    val rectangle = Rectangle(5.0, 2.0)
    println("The perimeter is ${rectangle.perimeter}")
}
```

Inheritance between classes is declared by a colon (`:`). Classes are `final` by default; to make a class inheritable, mark it as `open`:

```lin
open class Shape

class Rectangle(val height: Double, val length: Double): Shape() {
    val perimeter = (height + length) * 2 
}
```

## Comments

Dotlin supports `//` single-line (or _end-of-line_) and `/* ... */` multi-line (_block_) comments.

```lin
// This is an end-of-line comment

/* This is a block comment
   on multiple lines. */
```

Block comments in Dotlin can be nested:

```lin
/* The comment starts here
/* contains a nested comment *​/  
and ends here. */
```

## String templates

String interpolation with `$name` and `${expr}` is supported:

```lin
var a = 1
val s1 = "a is $a"
a = 2
val s2 = "${s1.replace("is", "was")}, but now is $a"
```

or better example:

```lin
fun main() {
    var a = 1
    // simple name in template:
    val s1 = "a is $a" 
    
    a = 2
    // arbitrary expression in template:
    val s2 = "${s1.replace("is", "was")}, but now is $a"
    println(s2)
}
```

## Conditional expressions

`if` can be used as a statement or an expression:

```lin
fun maxOf(a: Int, b: Int): Int {
    if (a > b) {
        return a
    } else {
        return b
    }
}

fun main() {
    println("max of 0 and 42 is ${maxOf(0, 42)}")
}
```

also `if` statements can be used as an expression:

```lin
fun maxOf(a: Int, b: Int): Int {
	if (a > b) return a else return b
}

fun maxOfExpr(a: Int, b: Int) = if (a > b) a else b

fun main() {
    println("max of 0 and 42 is ${maxOf(0, 42)}")
}
```

## Loops

### For loop

```lin
fun main() {
    val items = listOf("apple", "banana", "kiwifruit")
    for (item in items) {
        println(item)
    }
}
```

or:

```lin
fun main() {
    val items = listOf("apple", "banana", "kiwifruit")
    for (index in items.indices) {
        println("item at $index is ${items[index]}")
    }
}
```

#### `for` over collections and indices:

```lin
val items = listOf("apple", "banana", "kiwifruit")
for (item in items) println(item)
for (i in items.indices) println("item at $i is ${items[i]}")
```

### `while` loop:


```lin
fun main() {
    val items = listOf("apple", "banana", "kiwifruit")
    var index = 0
    while (index < items.size) {
        println("item at $index is ${items[index]}")
        index++
    }
}
```

another example of `while` loop:

```lin
var idx = 0
while (idx < items.size) {
	println(items[idx])
	idx++
}
```

### When expression

```lin
fun describe(obj: Any): String =
    when (obj) {
        1          -> "One"
        "Hello"    -> "Greeting"
        is Long    -> "Long"
        !is String -> "Not a string"
        else       -> "Unknown"
    }

fun main() {
    println(describe(1))
    println(describe("Hello"))
    println(describe(1000L))
    println(describe(2))
    println(describe("other"))
}
```

Pattern-like `when` expressions are supported:

```lin
fun describe(obj: Any): String = when (obj) {
	1 -> "One"
	"Hello" -> "Greeting"
	is Long -> "Long"
	!is String -> "Not a string"
	else -> "Unknown"
}
```

### Ranges and progressions

Check if a number is within a range using `in` operator:
```lin
fun main() {
    val x = 10
    val y = 9
    if (x in 1..y+1) {
        println("fits in range")
    }
}
```

Check if a number is out of range:

```lin
fun main() {
    val list = listOf("a", "b", "c")
    
    if (-1 !in 0..list.lastIndex) {
        println("-1 is out of range")
    }
    if (list.size !in list.indices) {
        println("list size is out of valid list indices range, too")
    }
}
```

Iterate over a range:

```lin
fun main() {
    for (x in 1..5) {
        print(x)
    }
}
```

Or over a progression:

```lin
fun main() {
    for (x in 1..10 step 2) {
        print(x)
    }
    println()
    for (x in 9 downTo 0 step 3) {
        print(x)
    }
}
```

Range checks and iteration:

```lin
if (10 in 1..9+1) println("fits")
for (x in 1..5) print(x)
for (x in 1..10 step 2) print(x)
for (x in 9 downTo 0 step 3) print(x)
```

## Collections and higher-order ops

Iterate over a collection:

```lin
fun main() {
    val items = listOf("apple", "banana", "kiwifruit")
    for (item in items) {
        println(item)
    }
}
```

Check if a collection contains an object using `in` operator:

```lin
fun main() {
    val items = setOf("apple", "banana", "kiwifruit")
    when {
        "orange" in items -> println("juicy")
        "apple" in items -> println("apple is fine too")
    }
}
```

Use [lambda expressions](lambdas.md) to filter and map collections:

```lin
fun main() {
    val fruits = listOf("banana", "avocado", "apple", "kiwifruit")
    fruits
      .filter { it.startsWith("a") }
      .sortedBy { it }
      .map { it.uppercase() }
      .forEach { println(it) }
}
```

Filtering, mapping, and iteration with lambdas:

```lin
val fruits = listOf("banana", "avocado", "apple", "kiwifruit")
fruits.filter { it.startsWith("a") }
	  .sortedBy { it }
	  .map { it.uppercase() }
	  .forEach { println(it) }
```

## Nullable types/values and null checks

A reference must be explicitly marked as nullable when `null` value is possible. Nullable type names have `?` at the end.

Return `null` if `str` does not hold an integer:

```lin
fun parseInt(str: String): Int? {
    // ...
}
```

Use a function returning nullable value:

```lin
fun parseInt(str: String): Int? {
    return str.toIntOrNull()
}

fun printProduct(arg1: String, arg2: String) {
    val x = parseInt(arg1)
    val y = parseInt(arg2)

    // Using `x * y` yields error because they may hold nulls.
    if (x != null && y != null) {
        // x and y are automatically cast to non-nullable after null check
        println(x * y)
    }
    else {
        println("'$arg1' or '$arg2' is not a number")
    }    
}

fun main() {
    printProduct("6", "7")
    printProduct("a", "7")
    printProduct("a", "b")
}
```

or:

```lin
fun parseInt(str: String): Int? {
    return str.toIntOrNull()
}

fun printProduct(arg1: String, arg2: String) {
    val x = parseInt(arg1)
    val y = parseInt(arg2)
    
    // ...
    if (x == null) {
        println("Wrong number format in arg1: '$arg1'")
        return
    }
    if (y == null) {
        println("Wrong number format in arg2: '$arg2'")
        return
    }

    // x and y are automatically cast to non-nullable after null check
    println(x * y)
}

fun main() {
    printProduct("6", "7")
    printProduct("a", "7")
    printProduct("99", "b")
}
```

Nullable types use `?` and must be checked for null before use:

```lin
fun parseInt(str: String): Int? { /* ... */ }

val x = parseInt("1")
val y = parseInt("2")
if (x != null && y != null) println(x * y)
```

## Type checks and automatic casts

The `is` operator checks if an expression is an instance of a type.
If an immutable local variable or property is checked for a specific type, there's no need to cast it explicitly:

```lin
fun getStringLength(obj: Any): Int? {
    if (obj is String) {
        // `obj` is automatically cast to `String` in this branch
        return obj.length
    }

    // `obj` is still of type `Any` outside of the type-checked branch
    return null
}

fun main() {
    fun printLength(obj: Any) {
        println("Getting the length of '$obj'. Result: ${getStringLength(obj) ?: "Error: The object is not a string"} ")
    }
    printLength("Incomprehensibilities")
    printLength(1000)
    printLength(listOf(Any()))
}
```

or:

```lin
fun getStringLength(obj: Any): Int? {
    if (obj !is String) return null

    // `obj` is automatically cast to `String` in this branch
    return obj.length
}

fun main() {
    fun printLength(obj: Any) {
        println("Getting the length of '$obj'. Result: ${getStringLength(obj) ?: "Error: The object is not a string"} ")
    }
    printLength("Incomprehensibilities")
    printLength(1000)
    printLength(listOf(Any()))
}
```

or even:

```lin
fun getStringLength(obj: Any): Int? {
    // `obj` is automatically cast to `String` on the right-hand side of `&&`
    if (obj is String && obj.length >= 0) {
        return obj.length
    }

    return null
}

fun main() {
    fun printLength(obj: Any) {
        println("Getting the length of '$obj'. Result: ${getStringLength(obj) ?: "Error: The object is not a string"} ")
    }
    printLength("Incomprehensibilities")
    printLength("")
    printLength(1000)
}
```

`is` checks a value's type and enables automatic casting within the checked branch:

```lin
fun getStringLength(obj: Any): Int? {
	if (obj is String) return obj.length
	return null
}
```

---

This file documents the minimal set of core syntax features that Dotlin should support initially. More detailed semantics, examples, and edge cases will be expanded in later language-spec pages.

