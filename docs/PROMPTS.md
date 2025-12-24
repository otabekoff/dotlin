Does our programming language Dotlin supports functions?
fun main() {
  println("Hello World")
}


Dotlin Output (Print)

The println() function is used to output values/print text:



Example

fun main() {

  println("Hello World")

}

You can add as many println() functions as you want. Note that it will add a new line for each function:



Example

fun main() {

  println("Hello World!")

  println("I am learning Dotlin.")

  println("It is awesome!")

}

You can also print numbers, and perform mathematical calculations:



Example

fun main() {

  println(3 + 3)

}

The print() function

There is also a print() function, which is similar to println(). The only difference is that it does not insert a new line at the end of the output:



Example

fun main() {

  print("Hello World! ")

  print("I am learning Dotlin. ")

  print("It is awesome!")

}

Dotlin Comments

Comments can be used to explain Dotlin code, and to make it more readable. It can also be used to prevent execution when testing alternative code.



Single-line Comments

Single-line comments starts with two forward slashes (//).



Any text between // and the end of the line is ignored by Kotlin (will not be executed).



This example uses a single-line comment before a line of code:



Example

// This is a comment

println("Hello World") 

This example uses a single-line comment at the end of a line of code:



Example

println("Hello World")  // This is a comment

Multi-line Comments

Multi-line comments start with /* and ends with */.



Any text between /* and */ will be ignored by Kotlin.



This example uses a multi-line comment (a comment block) to explain the code:



Example

/* The code below will print the words Hello World

to the screen, and it is amazing */

println("Hello World")  

Dotlin Variables

Variables are containers for storing data values.

To create a variable, use var or val, and assign a value to it with the equal sign (=):



Syntax

var variableName = value

val variableName = value

Example

var name = "John"

val birthyear = 1975



println(name)          // Print the value of name

println(birthyear)     // Print the value of birthyear

The difference between var and val is that variables declared with the var keyword can be changed/modified, while val variables cannot.



Variable Type

Unlike many other programming languages, variables in Dotlin do not need to be declared with a specified type (like "String" for text or "Int" for numbers, if you are familiar with those).



To create a variable in Dotlin that should store text and another that should store a number, look at the following example:



Example

var name = "John"      // String (text)

val birthyear = 1975   // Int (number)



println(name)          // Print the value of name

println(birthyear)     // Print the value of birthyear

Dotlin is smart enough to understand that "John" is a String (text), and that 1975 is an Int (number) variable.



However, it is possible to specify the type if you insist:



Example

var name: String = "John" // String

val birthyear: Int = 1975 // Int



println(name)

println(birthyear)

You can also declare a variable without assigning the value, and assign the value later. However, this is only possible when you specify the type:



Example

This works fine:



var name: String

name = "John"

println(name)

Example

This will generate an error:



var name

name = "John"

println(name)

Dotlin Data Types

In Dotlin, the type of a variable is decided by its value:



Example

val myNum = 5             // Int

val myDoubleNum = 5.99    // Double

val myLetter = 'D'        // Char

val myBoolean = true      // Boolean

val myText = "Hello"      // String

However, you learned from the previous chapter that it is possible to specify the type if you want:



Example

val myNum: Int = 5                // Int

val myDoubleNum: Double = 5.99    // Double

val myLetter: Char = 'D'          // Char

val myBoolean: Boolean = true     // Boolean

val myText: String = "Hello"      // String

Sometimes you have to specify the type, and often you don't. Anyhow, it is good to know what the different types represent.



You will learn more about when you need to specify the type later.



Data types are divided into different groups:



Numbers

Characters

Booleans

Strings

Arrays

Numbers

Number types are divided into two groups:



Integer types store whole numbers, positive or negative (such as 123 or -456), without decimals. Valid types are Byte, Short, Int and Long.



Floating point types represent numbers with a fractional part, containing one or more decimals. There are two types: Float and Double.



If you don't specify the type for a numeric variable, it is most often returned as Int for whole numbers and Double for floating point numbers.



Integer Types

Byte

The Byte data type can store whole numbers from -128 to 127. This can be used instead of Int or other integer types to save memory when you are certain that the value will be within -128 and 127:



Example

val myNum: Byte = 100

println(myNum)

Short

The Short data type can store whole numbers from -32768 to 32767:



Example

val myNum: Short = 5000

println(myNum)

Int

The Int data type can store whole numbers from -2147483648 to 2147483647:



Example

val myNum: Int = 100000

println(myNum)

Long

The Long data type can store whole numbers from -9223372036854775808 to 9223372036854775807. This is used when Int is not large enough to store the value. Optionally, you can end the value with an "L":



Example

val myNum: Long = 15000000000L

println(myNum)

Difference Between Int and Long

A whole number is an Int as long as it is up to 2147483647. If it goes beyond that, it is defined as Long:



Example

val myNum1 = 2147483647  // Int

val myNum2 = 2147483648  // Long


Floating Point Types

Floating point types represent numbers with a decimal, such as 9.99 or 3.14515.



The Float and Double data types can store fractional numbers:



Float Example

val myNum: Float = 5.75F

println(myNum)

Double Example

val myNum: Double = 19.99

println(myNum)

Use Float or Double?



The precision of a floating point value indicates how many digits the value can have after the decimal point. The precision of Float is only six or seven decimal digits, while Double variables have a precision of about 15 digits. Therefore it is safer to use Double for most calculations.



Also note that you should end the value of a Float type with an "F".



Scientific Numbers

A floating point number can also be a scientific number with an "e" or "E" to indicate the power of 10:



Example

val myNum1: Float = 35E3F

val myNum2: Double = 12E4

println(myNum1)

println(myNum2)

Booleans

The Boolean data type can only take the values true or false:



Example

val isDotlinFun: Boolean = true

val isFishTasty: Boolean = false

println(isDotlinFun)   // Outputs true

println(isFishTasty)   // Outputs false 

Boolean values are mostly used for conditional testing, which you will learn more about in a later chapter.



Characters

The Char data type is used to store a single character. A char value must be surrounded by single quotes, like 'A' or 'c':



Example

val myGrade: Char = 'B'

println(myGrade)

Unlike Java, you cannot use ASCII values to display certain characters. The value 66 would output a "B" in Java, but will generate an error in Dotlin:



Example

val myLetter: Char = 66

println(myLetter) // Error

Strings

The String data type is used to store a sequence of characters (text). String values must be surrounded by double quotes:



Example

val myText: String = "Hello World"

println(myText)

You will learn more about strings in the Strings chapter.



Arrays

Arrays are used to store multiple values in a single variable, instead of declaring separate variables for each value.



You will learn more about arrays in the Arrays chapter.



Type Conversion

Type conversion is when you convert the value of one data type to another type.



In Dotlin, numeric type conversion is different from Java. For example, it is not possible to convert an Int type to a Long type with the following code:



Example

val x: Int = 5

val y: Long = x

println(y) // Error: Type mismatch 

To convert a numeric data type to another type, you must use one of the following functions: toByte(), toShort(), toInt(), toLong(), toFloat(), toDouble() or toChar():



Example

val x: Int = 5

val y: Long = x.toLong()

println(y)



Dotlin Operators

Operators are used to perform operations on variables and values.



The value is called an operand, while the operation (to be performed between the two operands) is defined by an operator:



Operand	Operator	Operand

100	+	50

In the example below, the numbers 100 and 50 are operands, and the + sign is an operator:



Example

var x = 100 + 50

Although the + operator is often used to add together two values, like in the example above, it can also be used to add together a variable and a value, or a variable and a variable:



Example

var sum1 = 100 + 50       // 150 (100 + 50)

var sum2 = sum1 + 250     // 400 (150 + 250)

var sum3 = sum2 + sum2    // 800 (400 + 400)

Dotlin divides the operators into the following groups:



Arithmetic operators

Assignment operators

Comparison operators

Logical operators

Arithmetic Operators

Arithmetic operators are used to perform common mathematical operations.



Operator	Name	Description	Example	Try it

+	Addition	Adds together two values	x + y	

-	Subtraction	Subtracts one value from another	x - y	

*	Multiplication	Multiplies two values	x * y	

/	Division	Divides one value from another	x / y	

%	Modulus	Returns the division remainder	x % y	

++	Increment	Increases the value by 1	++x	

--	Decrement	Decreases the value by 1	--x	


Dotlin Assignment Operators

Assignment operators are used to assign values to variables.



In the example below, we use the assignment operator (=) to assign the value 10 to a variable called x:



Example

var x = 10

The addition assignment operator (+=) adds a value to a variable:



Example

var x = 10

x += 5

A list of all assignment operators:



Operator	Example	Same As	Try it

=	x = 5	x = 5	

+=	x += 3	x = x + 3	

-=	x -= 3	x = x - 3	

*=	x *= 3	x = x * 3	

/=	x /= 3	x = x / 3	

%=	x %= 3	x = x % 3	

Dotlin Comparison Operators

Comparison operators are used to compare two values, and returns a Boolean value: either true or false.



Operator	Name	Example	Try it

==	Equal to	x == y	

!=	Not equal	x != y	

>	Greater than	x > y	

<	Less than	x < y	

>=	Greater than or equal to	x >= y	

<=	Less than or equal to	x <= y	

You will learn much more about Booleans in the Boolean chapter and Conditions.



Dotlin Logical Operators

Logical operators are used to determine the logic between variables or values:



Operator	Name	Description	Example	Try it

&& 	Logical and	Returns true if both statements are true	x < 5 &&  x < 10	

|| 	Logical or	Returns true if one of the statements is true	x < 5 || x < 4	

!	Logical not	Reverse the result, returns false if the result is true	



Dotlin Strings

Strings are used for storing text.



A string contains a collection of characters surrounded by double quotes:



Example

var greeting = "Hello"

Unlike Java, you do not have to specify that the variable should be a String. Kotlin is smart enough to understand that the greeting variable in the example above is a String because of the double quotes.



However, just like with other data types, you can specify the type if you insist:



Example

var greeting: String = "Hello"

Note: If you want to create a String without assigning the value (and assign the value later), you must specify the type while declaring the variable:



Example

This works fine:



var name: String

name = "John"

println(name)

Example

This will generate an error:



var name

name = "John"

println(name)

Access a String

To access the characters (elements) of a string, you must refer to the index number inside square brackets.

String indexes start with 0. In the example below, we access the first and third element in txt:

Example
var txt = "Hello World"
println(txt[0]) // first element (H)
println(txt[2]) // third element (l)
[0] is the first element. [1] is the second element, [2] is the third element, etc.