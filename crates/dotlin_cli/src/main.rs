mod interpreter;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dotlin")]
#[command(version)]
#[command(about = "Minimal dotlin CLI placeholder", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print a greeting
    Hello {
        /// Optional name to greet
        name: Option<String>,
    },
    /// Run a .lin file
    Run {
        /// Path to .lin file
        file: String,
        /// Arguments to pass to the program
        #[arg(num_args = 0..)]
        args: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Hello { name }) => {
            let who = name.unwrap_or_else(|| "world".into());
            println!("Hello, {}!", who);
        }
        Some(Commands::Run { file, args }) => {
            // parse and run the file using dotlin_parser and a tiny interpreter
            if let Err(e) = run_file(&file, &args) {
                eprintln!("error running file: {}", e);
            }
        }
        None => {
            println!("dotlin: a tiny placeholder CLI. Try 'dotlinc hello' or 'dotlinc run <file>'");
        }
    }
}

fn run_file(path: &str, extras: &Vec<String>) -> Result<(), String> {
    use dotlin_parser::parse_to_ast;
    use std::fs;
    let src = fs::read_to_string(path).map_err(|e| format!("failed to read {}: {}", path, e))?;
    let ast = parse_to_ast(&src);

    // build Interpreter and function table
    let mut it = crate::interpreter::Interpreter::new();
    let mut main_fn: Option<(Vec<String>, Vec<dotlin_parser::ast::Stmt>)> = None;
    for n in ast {
        match n {
            dotlin_parser::ast::Node::Function {
                name,
                params,
                return_type,
                body,
                expr_body,
                ..
            } => {
                // register function (expr_body may be Some for expression-bodied functions)
                it.register_fn(
                    name.clone(),
                    params.clone(),
                    body.clone(),
                    expr_body.clone(),
                    return_type.clone(),
                );
                if name == "main" {
                    main_fn = Some((params.clone(), body.clone()));
                }
            }
            dotlin_parser::ast::Node::Variable {
                is_mut, name, init, ..
            } => {
                // if initializer present, evaluate it in current interpreter and store value
                if let Some(expr) = init {
                    match it.eval_expr(expr) {
                        Ok(v) => it.env.declare(name.clone(), Some(v), is_mut),
                        Err(e) => {
                            return Err(format!(
                                "failed to evaluate initializer for {}: {}",
                                name, e
                            ));
                        }
                    }
                } else {
                    it.env.declare(name.clone(), None, is_mut);
                }
            }
            _ => {}
        }
    }

    // convert provided extras to interpreter Values
    let mut arg_values = Vec::new();
    for e in extras.iter() {
        arg_values.push(crate::interpreter::Value::Str(e.clone()));
    }

    if let Some((params, body)) = main_fn {
        // if main expects params, pass them
        let _ = it.run_function(&params, &arg_values, body)?;
        return Ok(());
    }
    Err("no main function found".into())
}
