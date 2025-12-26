For Dotlin today:

Removing dotlin_runtime.lib is reasonable, clean, and professional.

Next steps I recommended (brief)

Add codegen unit tests that compile small snippets using .iter() with tuple destructuring and assert the generated program behavior.
Add an integration test that compiles + runs a compiled for (k, v) in m.iter() example.
Run the full workspace test suite.
Verify CI artifacts on Windows include both the DLL and the import-lib (or switch to static linking), and improve build.rs to reliably expose/copy the import lib for CI.
Do you want me to add those codegen tests and run the full test suite now? (I can implement tests, run cargo test --all, and check CI-artifact behavior; say “Yes” to proceed.)


Kotlin Syntax
Kotlin Output
Kotlin Comments
Kotlin Variables
Kotlin Data Types
Kotlin Operators
Kotlin Strings
Kotlin Booleans
Kotlin If...Else
Kotlin When
Kotlin While Loop
Kotlin Break/Continue
Kotlin Arrays
Kotlin For Loop
Kotlin Ranges
Kotlin Functions​
Kotlin Classes
Kotlin OOP
Kotlin Classes/Objects
Kotlin Constructors
Kotlin Class Functions
Kotlin Inheritance



​