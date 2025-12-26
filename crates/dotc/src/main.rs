use clap::Parser;
use dotlin_codegen::CodeGenerator;
use dotlin_interpreter::Interpreter;
use dotlin_parser::Parser as DotlinParser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file (.lin)
    #[arg(name = "INPUT")]
    input: Option<PathBuf>,

    /// Output file (.o or .exe)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Path to the directory containing dotlin_runtime.lib
    #[arg(long)]
    runtime_path: Option<PathBuf>,

    /// Only compile to object file, do not link
    #[arg(short, long)]
    compile_only: bool,

    /// Run the file using the interpreter instead of compiling
    #[arg(short, long)]
    run: bool,
}

fn main() {
    let cli = Cli::parse();

    if let Some(input_path) = &cli.input {
        if cli.run {
            // Run using interpreter instead of compiling
            run_with_interpreter(input_path);
        } else {
            // Compile using the original method
            let content = fs::read_to_string(input_path).expect("Failed to read file");
            let mut parser = DotlinParser::new(&content);

            match parser.parse_program() {
                Ok(mut ast) => {
                    let mut typechecker = dotlin_typechecker::TypeChecker::new();
                    if let Err(e) = typechecker.check_program(&mut ast) {
                        eprintln!("Type Error: {}", e);
                        return;
                    }

                    let generator = CodeGenerator::new();
                    match generator.compile_program(&ast) {
                        Ok(bytes) => {
                            let obj_path = if cli.compile_only {
                                cli.output.clone().unwrap_or(PathBuf::from("output.o"))
                            } else {
                                PathBuf::from("temp_output.o")
                            };

                            fs::write(&obj_path, bytes).expect("Failed to write object file");

                            if !cli.compile_only {
                                link_executable(&obj_path, &cli);
                                if obj_path.to_string_lossy() == "temp_output.o" {
                                    let _ = fs::remove_file(obj_path);
                                }
                            } else {
                                println!("Compiled to {:?}", obj_path);
                            }
                        }
                        Err(e) => eprintln!("Compilation Error: {}", e),
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing file: {}", e);
                }
            }
        }
    } else {
        println!("No input file provided.");
    }
}

fn link_executable(obj_path: &Path, cli: &Cli) {
    let exe_path = cli.output.clone().unwrap_or(PathBuf::from("output.exe"));

    // Create a temporary wrapper file
    let wrapper_path = PathBuf::from("temp_wrapper.rs");
    let wrapper_content = r#"
extern "C" {
    fn main_lin();
}
fn main() {
    unsafe { main_lin(); }
}
"#;
    fs::write(&wrapper_path, wrapper_content).expect("Failed to write wrapper");

    let mut cmd = Command::new("rustc");
    cmd.arg(&wrapper_path)
        .arg("-C")
        .arg(format!("link-arg={}", obj_path.display()))
        .arg("-o")
        .arg(&exe_path);

    // Determine runtime search dirs (cli override, then absolute workspace lib)
    let mut runtime_dirs: Vec<PathBuf> = Vec::new();
    if let Some(ref path) = cli.runtime_path {
        // Prefer canonicalized path, fall back to provided
        let rp = path.canonicalize().unwrap_or_else(|_| path.clone());
        runtime_dirs.push(rp);
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    runtime_dirs.push(cwd.clone());
    runtime_dirs.push(cwd.join("lib"));

    for d in &runtime_dirs {
        cmd.arg("-L").arg(d);
    }

    // Choose linking flavor depending on available runtime artifact
    let mut link_arg = None;
    if cfg!(target_os = "windows") {
        link_arg = Some("static=dotlin_runtime".to_string());
    } else {
        // Search for static archive or shared library in runtime_dirs
        for d in &runtime_dirs {
            let static_lib = d.join("libdotlin_runtime.a");
            if static_lib.exists() {
                link_arg = Some("static=dotlin_runtime".to_string());
                break;
            }
            let so_lib = d.join("libdotlin_runtime.so");
            if so_lib.exists() {
                link_arg = Some("dylib=dotlin_runtime".to_string());
                break;
            }
            let dylib_mac = d.join("libdotlin_runtime.dylib");
            if dylib_mac.exists() {
                link_arg = Some("dylib=dotlin_runtime".to_string());
                break;
            }
        }
        // Fallback to dynamic if nothing found; this lets the linker try default names.
        if link_arg.is_none() {
            link_arg = Some("dylib=dotlin_runtime".to_string());
        }
    }

    if let Some(arg) = link_arg {
        cmd.arg("-l").arg(arg);
    }

    println!("Linking correctly...");
    let status = cmd.status().expect("Failed to run rustc for linking");

    let _ = fs::remove_file(wrapper_path);

    if status.success() {
        println!("Successfully linked to {:?}", exe_path);
    } else {
        eprintln!("Linking failed");
    }
}

fn run_with_interpreter(input_path: &PathBuf) {
    match std::fs::read_to_string(input_path) {
        Ok(content) => {
            let mut parser = dotlin_parser::Parser::new(&content);
            match parser.parse_program() {
                Ok(mut ast) => {
                    // Type check
                    let mut typechecker = dotlin_typechecker::TypeChecker::new();
                    if let Err(e) = typechecker.check_program(&mut ast) {
                        eprintln!("Type Error: {}", e);
                        return;
                    }

                    // Run with interpreter
                    let mut interpreter = Interpreter::new();
                    match interpreter.interpret_program(&ast) {
                        Ok(()) => println!("Program executed successfully"),
                        Err(e) => eprintln!("Runtime Error: {}", e),
                    }
                }
                Err(e) => {
                    eprintln!("Parse Error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
