use dotlin_parser::{parse_to_ast, parse_to_tokens};
use std::fs;
use std::path::Path;

fn find_example() -> Option<String> {
    let candidates = [
        "examples/hello_world/main.lin",
        "../examples/hello_world/main.lin",
        "../../examples/hello_world/main.lin",
        "../../../examples/hello_world/main.lin",
    ];
    for c in &candidates {
        if Path::new(c).exists() {
            return Some(c.to_string());
        }
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        args[1].clone()
    } else {
        find_example().expect("could not find examples/hello_world/main.lin")
    };
    let src = fs::read_to_string(&path).expect("failed to read example file");

    println!("--- Source: {} ---\n{}\n", path, src);

    let toks = parse_to_tokens(&src);
    println!("--- Tokens (first 80) ---");
    for t in toks.iter().take(80) {
        println!("{:?}", t);
    }

    let ast = parse_to_ast(&src);
    println!("--- AST ---\n{:?}", ast);
}
