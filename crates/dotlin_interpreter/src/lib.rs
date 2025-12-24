use dotlin_ast::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Void,
    Function {
        declaration: FunctionDecl,
        closure: Rc<RefCell<Environment>>,
    },
    // Built-in functions later?
    NativeFunction(fn(Vec<Value>) -> Result<Value, RuntimeError>),
    Array(Vec<Value>),
    HashMap(std::collections::HashMap<String, Value>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Void, Value::Void) => true,
            (
                Value::Function {
                    declaration: d1, ..
                },
                Value::Function {
                    declaration: d2, ..
                },
            ) => d1 == d2,
            (Value::NativeFunction(f1), Value::NativeFunction(f2)) => {
                // Function pointers comparison is generally discouraged but here we need it for equality.
                // We can cast to usize or just allow it with logic.
                // But for now let's just say they are equal if addresses are equal, suppressing warning if possible or just implementing it.
                // Actually the warning says it is unpredictable.
                // Let's just return false for native functions equality for now, or use `ptr::eq` logic if really needed.
                // Or just cast to usize.
                *f1 as usize == *f2 as usize
            }
            (Value::Array(a1), Value::Array(a2)) => a1 == a2,
            (Value::HashMap(m1), Value::HashMap(m2)) => m1 == m2,
            _ => false,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Void => write!(f, "()"),
            Value::Function { declaration, .. } => write!(f, "fun {}", declaration.name),
            Value::NativeFunction(_) => write!(f, "<native fn>"),
            Value::Array(elements) => {
                write!(f, "Array(")?;
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", element)?;
                }
                write!(f, ")")
            },
            Value::HashMap(map) => {
                write!(f, "HashMap(")?;
                let mut iter = map.iter().enumerate();
                if let Some((_i, (key, value))) = iter.next() {
                    write!(f, "{}: {}", key, value)?;
                    for (_i, (key, value)) in iter {
                        write!(f, ", {}: {}", key, value)?;
                    }
                }
                write!(f, ")")
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Undefined variable '{0}'")]
    UndefinedVariable(String),
    #[error("Type mismatch: {0}")]
    TypeMismatch(String),
    #[error("Not a function: {0}")]
    NotAFunction(String),
    #[error("Incorrect argument count: expected {expected}, got {got}")]
    ArgumentCount { expected: usize, got: usize },
    #[error("Return statement outside function")]
    Return(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value, RuntimeError> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(RuntimeError::UndefinedVariable(name.to_string()))
        }
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError::UndefinedVariable(name))
        }
    }
}

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));

        // Define native println
        globals.borrow_mut().define(
            "println".to_string(),
            Value::NativeFunction(|args| {
                for arg in args {
                    print!("{} ", arg);
                }
                println!();
                Ok(Value::Void)
            }),
        );

        Self { globals }
    }

    pub fn interpret_program(&mut self, program: &Program) -> Result<(), RuntimeError> {
        for decl in &program.declarations {
            self.execute_declaration(decl, self.globals.clone())?;
        }

        // Look for main function and run it
        let main = self.globals.borrow().get("main");
        if let Ok(Value::Function {
            declaration: _,
            closure: _,
        }) = main
        {
            // We can't reuse execute_expression for this easily without mocking AST,
            // so let's just call it directly
            self.call_value(main.unwrap(), vec![])?;
        }

        Ok(())
    }

    pub fn interpret_declaration(&self, decl: &Declaration) -> Result<(), RuntimeError> {
        self.execute_declaration(decl, self.globals.clone())
    }

    pub fn interpret_statement(&self, stmt: &Statement) -> Result<Option<Value>, RuntimeError> {
        match stmt {
            Statement::Expression(expr) => {
                let val = self.evaluate_expression(expr, self.globals.clone())?;
                Ok(Some(val))
            }
            _ => {
                self.execute_statement(stmt, self.globals.clone())?;
                Ok(None)
            }
        }
    }

    fn execute_declaration(
        &self,
        decl: &Declaration,
        env: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        match decl {
            Declaration::Function(func) => {
                let function = Value::Function {
                    declaration: func.clone(),
                    closure: env.clone(),
                };
                env.borrow_mut().define(func.name.clone(), function);
                Ok(())
            }
        }
    }

    fn execute_statement(
        &self,
        stmt: &Statement,
        env: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        match stmt {
            Statement::Expression(expr) => {
                self.evaluate_expression(expr, env)?;
                Ok(())
            }
            Statement::VariableDecl {
                name,
                typ: _,
                initializer,
            } => {
                let value = if let Some(init) = initializer {
                    self.evaluate_expression(init, env.clone())?
                } else {
                    Value::Void // Or Error for uninitialized?
                };
                env.borrow_mut().define(name.clone(), value);
                Ok(())
            }
            Statement::Return(expr) => {
                let value = if let Some(e) = expr {
                    self.evaluate_expression(e, env)?
                } else {
                    Value::Void
                };
                Err(RuntimeError::Return(value))
            }
            Statement::Block(block) => {
                let new_env = Rc::new(RefCell::new(Environment::with_enclosing(env)));
                self.execute_block(block, new_env)
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = self.evaluate_expression(condition, env.clone())?;
                if let Value::Boolean(b) = cond {
                    if b {
                        self.execute_statement(then_branch, env)?;
                    } else if let Some(els) = else_branch {
                        self.execute_statement(els, env)?;
                    }
                    Ok(())
                } else {
                    Err(RuntimeError::TypeMismatch(
                        "Condition must be boolean".to_string(),
                    ))
                }
            }
            Statement::While { condition, body } => {
                loop {
                    let cond = self.evaluate_expression(condition, env.clone())?;
                    if let Value::Boolean(b) = cond {
                        if !b {
                            break;
                        }
                        self.execute_statement(body, env.clone())?;
                    } else {
                        return Err(RuntimeError::TypeMismatch(
                            "Condition must be boolean".to_string(),
                        ));
                    }
                }
                Ok(())
            }
        }
    }

    fn evaluate_expression(
        &self,
        expr: &Expression,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        match &*expr.kind {
            ExpressionKind::Literal(lit) => Ok(match lit {
                Literal::Integer(i) => Value::Integer(*i),
                Literal::Float(f) => Value::Float(*f),
                Literal::String(s) => Value::String(s.clone()),
                Literal::Boolean(b) => Value::Boolean(*b),
            }),
            ExpressionKind::Variable(name) => env.borrow().get(name),
            ExpressionKind::Assignment { name, value } => {
                let val = self.evaluate_expression(value, env.clone())?;
                env.borrow_mut().assign(name.clone(), val.clone())?;
                Ok(val)
            }
            ExpressionKind::Call { callee, arguments } => {
                let callee_val = self.evaluate_expression(callee, env.clone())?;
                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate_expression(arg, env.clone())?);
                }
                self.call_value(callee_val, args)
            }
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                let l = self.evaluate_expression(left, env.clone())?;
                let r = self.evaluate_expression(right, env.clone())?;
                self.evaluate_binary(l, operator, r)
            }
            ExpressionKind::Unary { operator, operand } => {
                let val = self.evaluate_expression(operand, env)?;
                match (operator, val) {
                    (UnaryOp::Minus, Value::Integer(i)) => Ok(Value::Integer(-i)),
                    (UnaryOp::Minus, Value::Float(f)) => Ok(Value::Float(-f)),
                    (UnaryOp::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
                    _ => Err(RuntimeError::TypeMismatch(
                        "Invalid operand for unary operator".to_string(),
                    )),
                }
            }
            ExpressionKind::MemberAccess { object, member } => {
                let obj_val = self.evaluate_expression(object, env)?;
                match (obj_val, member.as_str()) {
                    (Value::String(s), "length") => Ok(Value::Integer(s.len() as i64)),
                    (val, _) => Err(RuntimeError::TypeMismatch(format!(
                        "Cannot access member '{}' on {:?}",
                        member, val
                    ))),
                }
            }
            ExpressionKind::ArrayLiteral { elements } => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.evaluate_expression(element, env.clone())?);
                }
                Ok(Value::Array(values))
            }
            ExpressionKind::Index { array, index } => {
                let arr_val = self.evaluate_expression(array, env.clone())?;
                let idx_val = self.evaluate_expression(index, env)?;
                
                match (arr_val, idx_val) {
                    (Value::Array(elements), Value::Integer(index)) => {
                        let idx = index as usize;
                        if idx < elements.len() {
                            Ok(elements[idx].clone())
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                "Array index out of bounds".to_string(),
                            ))
                        }
                    }
                    (Value::HashMap(map), Value::String(key)) => {
                        match map.get(&key) {
                            Some(value) => Ok(value.clone()),
                            None => Err(RuntimeError::TypeMismatch(
                                format!("Key '{}' not found in HashMap", key),
                            )),
                        }
                    }
                    (Value::HashMap(map), Value::Integer(key)) => {
                        match map.get(&key.to_string()) {
                            Some(value) => Ok(value.clone()),
                            None => Err(RuntimeError::TypeMismatch(
                                format!("Key '{}' not found in HashMap", key),
                            )),
                        }
                    }
                    (_, Value::Integer(_)) => {
                        Err(RuntimeError::TypeMismatch(
                            "Indexing target is not an array".to_string(),
                        ))
                    }
                    (_, Value::String(_)) => {
                        Err(RuntimeError::TypeMismatch(
                            "Indexing target is not a HashMap".to_string(),
                        ))
                    }
                    _ => Err(RuntimeError::TypeMismatch(
                        "Index must be an integer for arrays or string for HashMaps".to_string(),
                    )),
                }
            }
            ExpressionKind::HashMapLiteral { pairs } => {
                let mut map = std::collections::HashMap::new();
                for (key_expr, value_expr) in pairs {
                    let key = self.evaluate_expression(key_expr, env.clone())?;
                    let value = self.evaluate_expression(value_expr, env.clone())?;
                    
                    // Convert key to string if it's a string literal
                    let key_str = match key {
                        Value::String(s) => s,
                        Value::Integer(i) => i.to_string(),
                        _ => return Err(RuntimeError::TypeMismatch("HashMap key must be string or integer".to_string())),
                    };
                    
                    map.insert(key_str, value);
                }
                Ok(Value::HashMap(map))
            }
        }
    }

    fn call_value(&self, callee: Value, args: Vec<Value>) -> Result<Value, RuntimeError> {
        match callee {
            Value::Function {
                declaration,
                closure,
            } => {
                if args.len() != declaration.params.len() {
                    return Err(RuntimeError::ArgumentCount {
                        expected: declaration.params.len(),
                        got: args.len(),
                    });
                }

                let environment = Rc::new(RefCell::new(Environment::with_enclosing(closure)));
                for (i, param) in declaration.params.iter().enumerate() {
                    environment
                        .borrow_mut()
                        .define(param.name.clone(), args[i].clone());
                }

                match self.execute_block(&declaration.body, environment) {
                    Ok(_) => Ok(Value::Void),
                    Err(RuntimeError::Return(val)) => Ok(val),
                    Err(e) => Err(e),
                }
            }
            Value::NativeFunction(f) => f(args),
            _ => Err(RuntimeError::NotAFunction(format!("{}", callee))),
        }
    }

    fn execute_block(
        &self,
        block: &Block,
        env: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        for stmt in &block.statements {
            self.execute_statement(stmt, env.clone())?;
        }
        Ok(())
    }

    fn evaluate_binary(
        &self,
        left: Value,
        op: &BinaryOp,
        right: Value,
    ) -> Result<Value, RuntimeError> {
        match (left, op, right) {
            (Value::Integer(l), BinaryOp::Add, Value::Integer(r)) => Ok(Value::Integer(l + r)),
            (Value::Integer(l), BinaryOp::Sub, Value::Integer(r)) => Ok(Value::Integer(l - r)),
            (Value::Integer(l), BinaryOp::Mul, Value::Integer(r)) => Ok(Value::Integer(l * r)),
            (Value::Integer(l), BinaryOp::Div, Value::Integer(r)) => Ok(Value::Integer(l / r)),

            // Comparisons
            (Value::Integer(l), BinaryOp::Equal, Value::Integer(r)) => Ok(Value::Boolean(l == r)),
            (Value::Integer(l), BinaryOp::NotEqual, Value::Integer(r)) => {
                Ok(Value::Boolean(l != r))
            }
            (Value::Integer(l), BinaryOp::Less, Value::Integer(r)) => Ok(Value::Boolean(l < r)),
            (Value::Integer(l), BinaryOp::LessEqual, Value::Integer(r)) => {
                Ok(Value::Boolean(l <= r))
            }
            (Value::Integer(l), BinaryOp::Greater, Value::Integer(r)) => Ok(Value::Boolean(l > r)),
            (Value::Integer(l), BinaryOp::GreaterEqual, Value::Integer(r)) => {
                Ok(Value::Boolean(l >= r))
            }

            // Boolean operations
            (Value::Boolean(l), BinaryOp::Equal, Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
            (Value::Boolean(l), BinaryOp::NotEqual, Value::Boolean(r)) => {
                Ok(Value::Boolean(l != r))
            }
            (Value::Boolean(l), BinaryOp::And, Value::Boolean(r)) => {
                Ok(Value::Boolean(l && r))
            }
            (Value::Boolean(l), BinaryOp::Or, Value::Boolean(r)) => {
                Ok(Value::Boolean(l || r))
            }

            // String concat
            (Value::String(l), BinaryOp::Add, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::String(l), BinaryOp::Add, Value::Integer(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::Integer(l), BinaryOp::Add, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::String(l), BinaryOp::Add, Value::Float(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::Float(l), BinaryOp::Add, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::String(l), BinaryOp::Add, Value::Boolean(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::Boolean(l), BinaryOp::Add, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            _ => Err(RuntimeError::TypeMismatch(
                "Binary operator operand mismatch".to_string(),
            )),
        }
    }
}
