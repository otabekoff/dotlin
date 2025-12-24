use std::fs;
use dotlin_lexer::Lexer;
use dotlin_parser::Parser;
use dotlin_interpreter::Interpreter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the example file
    let code = fs::read_to_string("examples/basic/hello.lin")?;
    println!("Running code:\n{}", code);
    
    // Tokenize
    let tokens: Vec<_> = Lexer::new(&code).collect();
    println!("Tokens: {:?}", tokens);
    
    // Parse
    let mut parser = Parser::new(&code);
    let mut program = parser.parse_program()?;
    println!("Parsed successfully");
    
    // Type check
    let mut typechecker = dotlin_typechecker::TypeChecker::new();
    typechecker.check_program(&mut program)?;
    println!("Type checked successfully");
    
    // Interpret
    let mut interpreter = Interpreter::new();
    interpreter.interpret_program(&program)?;
    println!("Executed successfully");
    
    Ok(())
}