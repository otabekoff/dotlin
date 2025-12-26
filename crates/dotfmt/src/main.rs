use clap::Parser as ClapParser;
use std::fs;
use anyhow::Result;
use dotlin_parser::Parser;
use dotlin_ast::*;

#[derive(ClapParser)]
#[command(name = "dotfmt")]
#[command(about = "Dotlin Code Formatter", long_about = None)]
struct Cli {
    /// Input file to format
    input: String,
    
    /// Output file (if not specified, overwrites input)
    #[arg(short, long)]
    output: Option<String>,
    
    /// Check formatting without modifying files
    #[arg(long)]
    check: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    
    // Read the input file
    let input_content = fs::read_to_string(&cli.input)?;
    
    // Format the content
    let formatted_content = format_code(&input_content)?;
    
    if cli.check {
        // Check if the content is already formatted
        if input_content == formatted_content {
            println!("File {} is already formatted", cli.input);
            std::process::exit(0);
        } else {
            println!("File {} needs formatting", cli.input);
            std::process::exit(1);
        }
    } else {
        // Write the formatted content
        let output_path = cli.output.as_ref().unwrap_or(&cli.input);
        fs::write(output_path, formatted_content)?;
        println!("Formatted {}", output_path);
    }
    
    Ok(())
}

fn format_code(input: &str) -> Result<String> {
    // Create a parser
    let mut parser = Parser::new(input);
    let program = parser.parse_program()?;
    
    // Format the AST back to code
    let formatted = format_program(&program);
    
    Ok(formatted)
}

fn format_program(program: &Program) -> String {
    let mut result = String::new();
    
    for decl in &program.declarations {
        result.push_str(&format_declaration(decl, 0));
        result.push_str("\n\n");
    }
    
    result.trim_end().to_string()
}

fn format_declaration(decl: &Declaration, indent_level: usize) -> String {
    let _indent = "    ".repeat(indent_level);
    
    match decl {
        Declaration::Function(func) => format_function(func, indent_level),
    }
}

fn format_function(func: &FunctionDecl, indent_level: usize) -> String {
    let indent = "    ".repeat(indent_level);
    
    let params_str = func.params
        .iter()
        .map(|param| format!("{}: {}", param.name, format_type(&param.typ)))
        .collect::<Vec<_>>()
        .join(", ");
    
    let return_str = if let Some(rt) = &func.return_type {
        format!(" -> {}", format_type(rt))
    } else {
        String::new()
    };
    
    let body_str = format_block(&func.body, indent_level + 1);
    
    format!("{}fun {}({}){} {{\n{}\n{}}}", indent, func.name, params_str, return_str, body_str, indent)
}

fn format_block(block: &Block, indent_level: usize) -> String {
    let mut result = String::new();
    let indent = "    ".repeat(indent_level);
    
    for stmt in &block.statements {
        result.push_str(&format!("{}{}\n", indent, format_statement(stmt, indent_level)));
    }
    
    result.trim_end().to_string()
}

fn format_statement(stmt: &Statement, indent_level: usize) -> String {
    match stmt {
        Statement::Expression(expr) => format_expression(expr, indent_level),
        Statement::Block(block) => {
            let indent = "    ".repeat(indent_level);
            format!("{{\n{}\n{}}}", format_block(block, indent_level + 1), indent)
        },
        Statement::VariableDecl { name, typ, initializer } => {
            let type_str = if let Some(t) = typ.as_ref() {
                format!(": {}", format_type(t))
            } else {
                String::new()
            };
            
            let init_str = if let Some(init) = initializer.as_ref() {
                format!(" = {}", format_expression(init, indent_level))
            } else {
                String::new()
            };
            
            format!("var {}{}{}", name, type_str, init_str)
        },
        Statement::Return(expr) => {
            let expr_str = if let Some(e) = expr.as_ref() {
                format!(" {}", format_expression(e, indent_level))
            } else {
                String::new()
            };
            
            format!("return{}", expr_str)
        },
        Statement::If { condition, then_branch, else_branch } => {
            let _indent = "    ".repeat(indent_level);
            let cond_str = format_expression(condition, indent_level);
            let then_str = format_statement(then_branch, indent_level);
            
            if let Some(else_stmt) = else_branch.as_ref() {
                let else_str = format_statement(else_stmt, indent_level);
                format!("if ({}) {} else {}", cond_str, then_str, else_str)
            } else {
                format!("if ({}) {}", cond_str, then_str)
            }
        },
        Statement::While { condition, body } => {
            let _indent = "    ".repeat(indent_level);
            let cond_str = format_expression(condition, indent_level);
            let body_str = format_statement(body, indent_level);
            
            format!("while ({}) {}", cond_str, body_str)
        },
        Statement::ForEach { variable, iterable, body } => {
            let _indent = "    ".repeat(indent_level);
            let iterable_str = format_expression(iterable, indent_level);
            let body_str = format_statement(body, indent_level);

            let var_str = match variable {
                dotlin_ast::ForEachTarget::Ident(s) => s.clone(),
                dotlin_ast::ForEachTarget::Tuple(vs) => format!("({})", vs.join(", ")),
            };

            format!("for {} in {} {}", var_str, iterable_str, body_str)
        },
    }
}

fn format_expression(expr: &Expression, _indent_level: usize) -> String {
    match &*expr.kind {
        ExpressionKind::Literal(lit) => format_literal(lit),
        ExpressionKind::Variable(name) => name.clone(),
        ExpressionKind::Assignment { name, value } => {
            format!("{} = {}", name, format_expression(value, _indent_level))
        },
        ExpressionKind::Call { callee, arguments } => {
            let args: Vec<String> = arguments.iter()
                .map(|arg| format_expression(arg, _indent_level))
                .collect();
            format!("{}({})", format_expression(callee, _indent_level), args.join(", "))
        },
        ExpressionKind::Binary { left, operator, right } => {
            format!("({} {} {})", 
                format_expression(left, _indent_level),
                format_binary_op(operator),
                format_expression(right, _indent_level)
            )
        },
        ExpressionKind::Unary { operator, operand } => {
            format!("{}{}", format_unary_op(operator), format_expression(operand, _indent_level))
        },
        ExpressionKind::MemberAccess { object, member } => {
            format!("{}.{}", format_expression(object, _indent_level), member)
        },
        ExpressionKind::ArrayLiteral { elements } => {
            let elts: Vec<String> = elements.iter()
                .map(|elt| format_expression(elt, _indent_level))
                .collect();
            format!("[{}]", elts.join(", "))
        },
        ExpressionKind::Index { array, index } => {
            format!("{}[{}]", format_expression(array, _indent_level), format_expression(index, _indent_level))
        },
        ExpressionKind::HashMapLiteral { pairs } => {
            let pairs_str: Vec<String> = pairs.iter()
                .map(|(key, value)| format!("{}: {}", format_expression(key, _indent_level), format_expression(value, _indent_level)))
                .collect();
            format!("{{ {} }}", pairs_str.join(", "))
        },
    }
}

fn format_literal(lit: &Literal) -> String {
    match lit {
        Literal::Integer(i) => i.to_string(),
        Literal::Float(f) => f.to_string(),
        Literal::String(s) => format!("\"{}\"", s),
        Literal::Boolean(b) => b.to_string(),
        Literal::Char(c) => format!("'{}'", c),
    }
}

fn format_type(typ: &Type) -> String {
    match typ {
        Type::Named(name) => name.clone(),
        Type::Array(inner) => format!("Array<{}>", format_type(inner)),
        Type::Map(key_type, value_type) => format!("Map<{}, {}>", format_type(key_type), format_type(value_type)),
        Type::Generic(name, params) => {
            let param_strs: Vec<String> = params.iter().map(format_type).collect();
            format!("{}<{}>", name, param_strs.join(", "))
        }
    }
}

fn format_binary_op(op: &BinaryOp) -> String {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Div => "/",
        BinaryOp::PlusEqual => "+=",
        BinaryOp::MinusEqual => "-=",
        BinaryOp::StarEqual => "*=",
        BinaryOp::SlashEqual => "/=",
        BinaryOp::Equal => "==",
        BinaryOp::NotEqual => "!=",
        BinaryOp::Less => "<",
        BinaryOp::LessEqual => "<=",
        BinaryOp::Greater => ">",
        BinaryOp::GreaterEqual => ">=",
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
    }.to_string()
}

fn format_unary_op(op: &UnaryOp) -> String {
    match op {
        UnaryOp::Not => "!",
        UnaryOp::Minus => "-",
        UnaryOp::Increment => "++",
        UnaryOp::Decrement => "--",
    }.to_string()
}
