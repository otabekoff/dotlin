use clap::Parser;
use dotlin_codegen::CodeGenerator;
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
}

fn main() {
    let cli = Cli::parse();

    if let Some(input_path) = &cli.input {
        let content = fs::read_to_string(&input_path).expect("Failed to read file");
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

    if let Some(ref path) = cli.runtime_path {
        cmd.arg("-L").arg(path);
    } else {
        // Try some default locations
        cmd.arg("-L").arg(".");
        cmd.arg("-L").arg("lib");
    }

    cmd.arg("-l").arg("static=dotlin_runtime");

    println!("Linking correctly...");
    let status = cmd.status().expect("Failed to run rustc for linking");

    let _ = fs::remove_file(wrapper_path);

    if status.success() {
        println!("Successfully linked to {:?}", exe_path);
    } else {
        eprintln!("Linking failed");
    }
}
