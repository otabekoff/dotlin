use clap::Parser as ClapParser;
use dotlin_interpreter::{Interpreter, Value};
use dotlin_parser::{Parser, ReplNode};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Cli {}

fn main() -> rustyline::Result<()> {
    let _cli = Cli::parse();
    let interpreter = Interpreter::new();
    let mut rl = DefaultEditor::new()?;

    println!("Dotlin REPL v0.1.0");
    println!("Type 'exit' or Ctrl-D to quit.");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                rl.add_history_entry(trimmed)?;
                if trimmed == "exit" {
                    break;
                }

                let mut parser = Parser::new(trimmed);
                match parser.parse_repl_input() {
                    Ok(node) => {
                        let res = match node {
                            ReplNode::Decl(decl) => {
                                interpreter.interpret_declaration(&decl).map(|_| None)
                            }
                            ReplNode::Stmt(stmt) => interpreter.interpret_statement(&stmt),
                        };

                        match res {
                            Ok(Some(val)) => {
                                if val != Value::Void {
                                    println!("= {}", val);
                                }
                            }
                            Ok(None) => {}
                            Err(e) => eprintln!("Runtime Error: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Parse Error: {}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
