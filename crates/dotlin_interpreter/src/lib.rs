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
    Char(char),
    Void,
    Function {
        declaration: FunctionDecl,
        closure: Rc<RefCell<Environment>>,
    },
    // Built-in functions later?
    NativeFunction(fn(Vec<Value>) -> Result<Value, RuntimeError>),
    Array(Vec<Value>),
    HashMap(std::collections::HashMap<String, Value>),
    Iterator(Rc<RefCell<IteratorState>>),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hashmap_iter_for_each_runs() {
        // Build AST for:
        // fun main() {
        //   var m = {"a":1, "b":2}
        //   var s = 0
        //   for (k, v) in m.iter() { s = s + v }
        // }

        let map_pairs = vec![
            (
                Expression::new(ExpressionKind::Literal(Literal::String("a".to_string()))),
                Expression::new(ExpressionKind::Literal(Literal::Integer(1))),
            ),
            (
                Expression::new(ExpressionKind::Literal(Literal::String("b".to_string()))),
                Expression::new(ExpressionKind::Literal(Literal::Integer(2))),
            ),
        ];

        let m_decl = Statement::VariableDecl {
            name: "m".to_string(),
            typ: None,
            initializer: Some(Expression::new(ExpressionKind::HashMapLiteral { pairs: map_pairs })),
        };

        let s_decl = Statement::VariableDecl {
            name: "s".to_string(),
            typ: None,
            initializer: Some(Expression::new(ExpressionKind::Literal(Literal::Integer(0)))),
        };

        // m.iter() call
        let call_iter = Expression::new(ExpressionKind::Call {
            callee: Expression::new(ExpressionKind::MemberAccess {
                object: Expression::new(ExpressionKind::Variable("m".to_string())),
                member: "iter".to_string(),
            }),
            arguments: vec![],
        });

        // for (k, v) in m.iter() { s = s + v }
        let body_stmt = Statement::Expression(Expression::new(ExpressionKind::Assignment {
            name: "s".to_string(),
            value: Expression::new(ExpressionKind::Binary {
                left: Expression::new(ExpressionKind::Variable("s".to_string())),
                operator: BinaryOp::Add,
                right: Expression::new(ExpressionKind::Variable("v".to_string())),
            }),
        }));

        let for_stmt = Statement::ForEach {
            variable: dotlin_ast::ForEachTarget::Tuple(vec!["k".to_string(), "v".to_string()]),
            iterable: call_iter,
            body: Box::new(body_stmt),
        };

        let func = FunctionDecl {
            name: "main".to_string(),
            params: vec![],
            return_type: None,
            body: Block { statements: vec![m_decl, s_decl, for_stmt] },
        };

        let program = Program { declarations: vec![Declaration::Function(func)] };

        let mut interp = Interpreter::new();
        let res = interp.interpret_program(&program);
        if let Err(e) = &res {
            eprintln!("Interpreter error: {:?}", e);
        }
        assert!(res.is_ok());
    }

    #[test]
    fn iterator_next_and_exhaustion() {
        // Build statements at top-level so variables live in globals
        // var m = {"a":1, "b":2}
        let map_pairs = vec![
            (
                Expression::new(ExpressionKind::Literal(Literal::String("a".to_string()))),
                Expression::new(ExpressionKind::Literal(Literal::Integer(1))),
            ),
            (
                Expression::new(ExpressionKind::Literal(Literal::String("b".to_string()))),
                Expression::new(ExpressionKind::Literal(Literal::Integer(2))),
            ),
        ];
        let m_decl = Statement::VariableDecl {
            name: "m".to_string(),
            typ: None,
            initializer: Some(Expression::new(ExpressionKind::HashMapLiteral { pairs: map_pairs })),
        };

        // var it = m.iter()
        let call_iter = Expression::new(ExpressionKind::Call {
            callee: Expression::new(ExpressionKind::MemberAccess {
                object: Expression::new(ExpressionKind::Variable("m".to_string())),
                member: "iter".to_string(),
            }),
            arguments: vec![],
        });
        let it_decl = Statement::VariableDecl {
            name: "it".to_string(),
            typ: None,
            initializer: Some(call_iter),
        };

        // var e1 = it.next()
        let call_next1 = Expression::new(ExpressionKind::Call {
            callee: Expression::new(ExpressionKind::MemberAccess {
                object: Expression::new(ExpressionKind::Variable("it".to_string())),
                member: "next".to_string(),
            }),
            arguments: vec![],
        });
        let e1_decl = Statement::VariableDecl {
            name: "e1".to_string(),
            typ: None,
            initializer: Some(call_next1),
        };

        // var e2 = it.next()
        let call_next2 = Expression::new(ExpressionKind::Call {
            callee: Expression::new(ExpressionKind::MemberAccess {
                object: Expression::new(ExpressionKind::Variable("it".to_string())),
                member: "next".to_string(),
            }),
            arguments: vec![],
        });
        let e2_decl = Statement::VariableDecl {
            name: "e2".to_string(),
            typ: None,
            initializer: Some(call_next2),
        };

        // var e3 = it.next()
        let call_next3 = Expression::new(ExpressionKind::Call {
            callee: Expression::new(ExpressionKind::MemberAccess {
                object: Expression::new(ExpressionKind::Variable("it".to_string())),
                member: "next".to_string(),
            }),
            arguments: vec![],
        });
        let e3_decl = Statement::VariableDecl {
            name: "e3".to_string(),
            typ: None,
            initializer: Some(call_next3),
        };

        let interp = Interpreter::new();
        // execute statements in globals
        interp.interpret_statement(&m_decl).unwrap();
        interp.interpret_statement(&it_decl).unwrap();
        interp.interpret_statement(&e1_decl).unwrap();
        interp.interpret_statement(&e2_decl).unwrap();
        interp.interpret_statement(&e3_decl).unwrap();

        // Inspect globals
        let g = interp.globals.borrow();
        let e1 = g.get("e1").unwrap();
        let e2 = g.get("e2").unwrap();
        let e3 = g.get("e3").unwrap();

        // e1 and e2 should be arrays of length 2, e3 should be Void
        if let Value::Array(a) = e1 {
            assert_eq!(a.len(), 2);
            match (&a[0], &a[1]) {
                (Value::String(_), Value::Integer(_)) => {}
                _ => panic!("e1 entry shape unexpected"),
            }
        } else {
            panic!("e1 expected array");
        }

        if let Value::Array(a) = e2 {
            assert_eq!(a.len(), 2);
            match (&a[0], &a[1]) {
                (Value::String(_), Value::Integer(_)) => {}
                _ => panic!("e2 entry shape unexpected"),
            }
        } else {
            panic!("e2 expected array");
        }

        assert!(matches!(e3, Value::Void));
    }

    #[test]
    fn iterator_next_on_empty_map_returns_void() {
        // var m = {}
        let map_pairs: Vec<(Expression, Expression)> = vec![];
        let m_decl = Statement::VariableDecl {
            name: "m".to_string(),
            typ: None,
            initializer: Some(Expression::new(ExpressionKind::HashMapLiteral { pairs: map_pairs })),
        };

        // var it = m.iter()
        let call_iter = Expression::new(ExpressionKind::Call {
            callee: Expression::new(ExpressionKind::MemberAccess {
                object: Expression::new(ExpressionKind::Variable("m".to_string())),
                member: "iter".to_string(),
            }),
            arguments: vec![],
        });
        let it_decl = Statement::VariableDecl {
            name: "it".to_string(),
            typ: None,
            initializer: Some(call_iter),
        };

        // var e1 = it.next()
        let call_next1 = Expression::new(ExpressionKind::Call {
            callee: Expression::new(ExpressionKind::MemberAccess {
                object: Expression::new(ExpressionKind::Variable("it".to_string())),
                member: "next".to_string(),
            }),
            arguments: vec![],
        });
        let e1_decl = Statement::VariableDecl {
            name: "e1".to_string(),
            typ: None,
            initializer: Some(call_next1),
        };

        let interp = Interpreter::new();
        interp.interpret_statement(&m_decl).unwrap();
        interp.interpret_statement(&it_decl).unwrap();
        interp.interpret_statement(&e1_decl).unwrap();

        let g = interp.globals.borrow();
        let e1 = g.get("e1").unwrap();
        assert!(matches!(e1, Value::Void));
    }
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
            (Value::Iterator(a), Value::Iterator(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IteratorState {
    pub items: Vec<Value>,
    pub pos: usize,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Char(c) => write!(f, "'{}'", c),
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
            Value::Iterator(_) => write!(f, "<iterator>"),
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

#[derive(Debug)]
pub enum RuntimeError {
    UndefinedVariable(String),
    TypeMismatch(String),
    NotAFunction(String),
    ArgumentCount { expected: usize, got: usize },
    Return(Box<Value>),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::UndefinedVariable(name) => write!(f, "Undefined variable '{}'", name),
            RuntimeError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            RuntimeError::NotAFunction(name) => write!(f, "Not a function: {}", name),
            RuntimeError::ArgumentCount { expected, got } => write!(f, "Incorrect argument count: expected {}, got {}", expected, got),
            RuntimeError::Return(_) => write!(f, "Return statement outside function"),
        }
    }
}

impl std::error::Error for RuntimeError {}

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
        if let Some(slot) = self.values.get_mut(&name) {
            *slot = value;
            Ok(())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError::UndefinedVariable(name))
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
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

        // Define math functions
        globals.borrow_mut().define(
            "abs".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err(RuntimeError::ArgumentCount { expected: 1, got: args.len() });
                }
                match &args[0] {
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    Value::Integer(i) => Ok(Value::Integer(i.abs())),
                    _ => Err(RuntimeError::TypeMismatch("abs() expects a number".to_string())),
                }
            }),
        );

        globals.borrow_mut().define(
            "min".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 2 {
                    return Err(RuntimeError::ArgumentCount { expected: 2, got: args.len() });
                }
                match (&args[0], &args[1]) {
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.min(*b))),
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer((*a).min(*b))),
                    (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.min(*b as f64))),
                    (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((*a as f64).min(*b))),
                    _ => Err(RuntimeError::TypeMismatch("min() expects two numbers".to_string())),
                }
            }),
        );

        globals.borrow_mut().define(
            "max".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 2 {
                    return Err(RuntimeError::ArgumentCount { expected: 2, got: args.len() });
                }
                match (&args[0], &args[1]) {
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(*b))),
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer((*a).max(*b))),
                    (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.max(*b as f64))),
                    (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((*a as f64).max(*b))),
                    _ => Err(RuntimeError::TypeMismatch("max() expects two numbers".to_string())),
                }
            }),
        );

        globals.borrow_mut().define(
            "sqrt".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err(RuntimeError::ArgumentCount { expected: 1, got: args.len() });
                }
                match &args[0] {
                    Value::Float(f) => {
                        if *f < 0.0 {
                            Err(RuntimeError::TypeMismatch("sqrt() expects a non-negative number".to_string()))
                        } else {
                            Ok(Value::Float(f.sqrt()))
                        }
                    },
                    Value::Integer(i) => {
                        if *i < 0 {
                            Err(RuntimeError::TypeMismatch("sqrt() expects a non-negative number".to_string()))
                        } else {
                            Ok(Value::Float((*i as f64).sqrt()))
                        }
                    },
                    _ => Err(RuntimeError::TypeMismatch("sqrt() expects a number".to_string())),
                }
            }),
        );

        globals.borrow_mut().define(
            "pow".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 2 {
                    return Err(RuntimeError::ArgumentCount { expected: 2, got: args.len() });
                }
                match (&args[0], &args[1]) {
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
                    (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.powf(*b as f64))),
                    (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((*a as f64).powf(*b))),
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Float((*a as f64).powf(*b as f64))),
                    _ => Err(RuntimeError::TypeMismatch("pow() expects two numbers".to_string())),
                }
            }),
        );

        globals.borrow_mut().define(
            "sin".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err(RuntimeError::ArgumentCount { expected: 1, got: args.len() });
                }
                match &args[0] {
                    Value::Float(f) => Ok(Value::Float(f.sin())),
                    Value::Integer(i) => Ok(Value::Float((*i as f64).sin())),
                    _ => Err(RuntimeError::TypeMismatch("sin() expects a number".to_string())),
                }
            }),
        );

        globals.borrow_mut().define(
            "cos".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err(RuntimeError::ArgumentCount { expected: 1, got: args.len() });
                }
                match &args[0] {
                    Value::Float(f) => Ok(Value::Float(f.cos())),
                    Value::Integer(i) => Ok(Value::Float((*i as f64).cos())),
                    _ => Err(RuntimeError::TypeMismatch("cos() expects a number".to_string())),
                }
            }),
        );

        globals.borrow_mut().define(
            "tan".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err(RuntimeError::ArgumentCount { expected: 1, got: args.len() });
                }
                match &args[0] {
                    Value::Float(f) => Ok(Value::Float(f.tan())),
                    Value::Integer(i) => Ok(Value::Float((*i as f64).tan())),
                    _ => Err(RuntimeError::TypeMismatch("tan() expects a number".to_string())),
                }
            }),
        );
        


        Self { globals }
    }

}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
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
                Err(RuntimeError::Return(Box::new(value)))
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
            Statement::ForEach { variable, iterable, body } => {
                let iterable_val = self.evaluate_expression(iterable, env.clone())?;

                match iterable_val {
                    Value::Array(elements) => {
                        for element in elements {
                            let new_env = Rc::new(RefCell::new(Environment::with_enclosing(env.clone())));
                            match variable {
                                dotlin_ast::ForEachTarget::Ident(name) => {
                                    new_env.borrow_mut().define(name.clone(), element);
                                }
                                dotlin_ast::ForEachTarget::Tuple(names) => {
                                    // Expect each element to be an array to destructure
                                    if let Value::Array(inner) = element {
                                        if inner.len() != names.len() {
                                            return Err(RuntimeError::TypeMismatch(
                                                "Destructuring assignment length mismatch".to_string(),
                                            ));
                                        }
                                        for (i, n) in names.iter().enumerate() {
                                            new_env.borrow_mut().define(n.clone(), inner[i].clone());
                                        }
                                    } else {
                                        return Err(RuntimeError::TypeMismatch(
                                            "Destructuring target requires array elements".to_string(),
                                        ));
                                    }
                                }
                            }
                            self.execute_statement(body, new_env)?;
                        }
                    },
                    Value::HashMap(map) => {
                        // If destructuring into a tuple of (key, value), iterate entries
                        if let dotlin_ast::ForEachTarget::Tuple(names) = variable {
                            if names.len() != 2 {
                                return Err(RuntimeError::TypeMismatch(
                                    "HashMap entries destructuring requires two variables".to_string(),
                                ));
                            }
                            for (key, value) in map.iter() {
                                let new_env = Rc::new(RefCell::new(Environment::with_enclosing(env.clone())));
                                new_env.borrow_mut().define(names[0].clone(), Value::String(key.clone()));
                                new_env.borrow_mut().define(names[1].clone(), value.clone());
                                self.execute_statement(body, new_env)?;
                            }
                        } else if let dotlin_ast::ForEachTarget::Ident(name) = variable {
                            // For HashMap iteration, iterate over the keys
                            for key in map.keys() {
                                let new_env = Rc::new(RefCell::new(Environment::with_enclosing(env.clone())));
                                new_env.borrow_mut().define(name.clone(), Value::String(key.clone()));
                                self.execute_statement(body, new_env)?;
                            }
                        }
                    },
                    Value::Iterator(it_rc) => {
                        loop {
                            let next_opt = {
                                let mut it = it_rc.borrow_mut();
                                if it.pos >= it.items.len() {
                                    None
                                } else {
                                    let v = it.items[it.pos].clone();
                                    it.pos += 1;
                                    Some(v)
                                }
                            };

                            if next_opt.is_none() {
                                break;
                            }

                            let element = next_opt.unwrap();
                            let new_env = Rc::new(RefCell::new(Environment::with_enclosing(env.clone())));
                            match variable {
                                dotlin_ast::ForEachTarget::Ident(name) => {
                                    new_env.borrow_mut().define(name.clone(), element);
                                }
                                dotlin_ast::ForEachTarget::Tuple(names) => {
                                    if let Value::Array(inner) = element {
                                        if inner.len() != names.len() {
                                            return Err(RuntimeError::TypeMismatch(
                                                "Destructuring assignment length mismatch".to_string(),
                                            ));
                                        }
                                        for (i, n) in names.iter().enumerate() {
                                            new_env.borrow_mut().define(n.clone(), inner[i].clone());
                                        }
                                    } else {
                                        return Err(RuntimeError::TypeMismatch(
                                            "Destructuring target requires array elements".to_string(),
                                        ));
                                    }
                                }
                            }
                            self.execute_statement(body, new_env)?;
                        }
                    },
                    _ => {
                        return Err(RuntimeError::TypeMismatch(
                            "ForEach iterable must be an array or HashMap".to_string(),
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
                Literal::Char(c) => Value::Char(*c),
            }),
            ExpressionKind::Variable(name) => env.borrow().get(name),
            ExpressionKind::Assignment { name, value } => {
                let val = self.evaluate_expression(value, env.clone())?;
                env.borrow_mut().assign(name.clone(), val.clone())?;
                Ok(val)
            }
            ExpressionKind::Call { callee, arguments } => {
                // Handle method calls (obj.method()) which are represented as Call with MemberAccess callee
                if let ExpressionKind::MemberAccess { object, member } = &*callee.kind {
                    // This is a method call on an object
                    let obj_val = self.evaluate_expression(object, env.clone())?;
                    let _args: Vec<Value> = arguments
                        .iter()
                        .map(|arg| self.evaluate_expression(arg, env.clone()))
                        .collect::<Result<Vec<_>, _>>()?;
                    
                    // Handle type conversion methods and HashMap iteration methods
                    match (obj_val, member.as_str()) {
                        (Value::String(s), "toInt") => {
                            match s.parse::<i64>() {
                                Ok(num) => Ok(Value::Integer(num)),
                                Err(_) => Err(RuntimeError::TypeMismatch(
                                    format!("Cannot convert string '{}' to integer", s)
                                )),
                            }
                        },
                        (Value::String(s), "toFloat") => {
                            match s.parse::<f64>() {
                                Ok(num) => Ok(Value::Float(num)),
                                Err(_) => Err(RuntimeError::TypeMismatch(
                                    format!("Cannot convert string '{}' to float", s)
                                )),
                            }
                        },
                        (Value::Integer(n), "toFloat") => Ok(Value::Float(n as f64)),
                        (Value::Float(f), "toInt") => Ok(Value::Integer(f as i64)),
                        (Value::Integer(n), "toString") => Ok(Value::String(n.to_string())),
                        (Value::Float(f), "toString") => Ok(Value::String(f.to_string())),
                        (Value::Boolean(b), "toString") => Ok(Value::String(b.to_string())),
                        (Value::Char(c), "toString") => Ok(Value::String(c.to_string())),
                        // Array methods
                        (Value::Array(mut elements), "push") => {
                            if _args.len() != 1 {
                                return Err(RuntimeError::TypeMismatch("push() expects 1 argument".to_string()));
                            }
                            elements.push(_args[0].clone());
                            // Return void (or the new length if we want to follow JS conventions)
                            Ok(Value::Array(elements))
                        },
                        (Value::Array(mut elements), "pop") => {
                            if elements.is_empty() {
                                return Err(RuntimeError::TypeMismatch("pop() called on empty array".to_string()));
                            }
                            let last_element = elements.pop().unwrap();
                            Ok(last_element)
                        },
                        // HashMap iteration methods
                        (Value::HashMap(map), "keys") => {
                            let keys: Vec<Value> = map.keys().map(|k| Value::String(k.clone())).collect();
                            Ok(Value::Array(keys))
                        },
                        (Value::HashMap(map), "iter") => {
                            let mut entries = Vec::new();
                            for (key, value) in map.iter() {
                                let entry = Value::Array(vec![Value::String(key.clone()), value.clone()]);
                                entries.push(entry);
                            }
                            let it = IteratorState { items: entries, pos: 0 };
                            Ok(Value::Iterator(Rc::new(RefCell::new(it))))
                        },
                        (Value::HashMap(map), "values") => {
                            let values: Vec<Value> = map.values().cloned().collect();
                            Ok(Value::Array(values))
                        },
                        (Value::HashMap(map), "size") => {
                            Ok(Value::Integer(map.len() as i64))
                        },
                        (Value::HashMap(map), "entries") => {
                            let mut entries = Vec::new();
                            for (key, value) in map.iter() {
                                // Create an array with [key, value] for each entry
                                let entry = Value::Array(vec![Value::String(key.clone()), value.clone()]);
                                entries.push(entry);
                            }
                            Ok(Value::Array(entries))
                        },
                        // Iterator next() on iterator objects
                        (Value::Iterator(it_rc), "next") => {
                            let mut it = it_rc.borrow_mut();
                            if it.pos >= it.items.len() {
                                Ok(Value::Void)
                            } else {
                                let v = it.items[it.pos].clone();
                                it.pos += 1;
                                Ok(v)
                            }
                        },
                        (obj_val, method_name) => Err(RuntimeError::TypeMismatch(format!(
                            "Method '{}' not found on type {:?}",
                            method_name, obj_val
                        )))
                    }
                } else {
                    // Regular function call
                    let callee_val = self.evaluate_expression(callee, env.clone())?;
                    let mut args = Vec::new();
                    for arg in arguments {
                        args.push(self.evaluate_expression(arg, env.clone())?);
                    }
                    self.call_value(callee_val, args)
                }
            }
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                match operator {
                    // Handle logical operators with short-circuit evaluation
                    BinaryOp::And => {
                        let l_val = self.evaluate_expression(left, env.clone())?;
                        if let Value::Boolean(false) = l_val {
                            // If left is false, return false without evaluating right (short-circuit)
                            Ok(Value::Boolean(false))
                        } else {
                            // If left is true, evaluate and return the right operand
                            let r_val = self.evaluate_expression(right, env.clone())?;
                            match (l_val, r_val) {
                                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l && r)),
                                _ => Err(RuntimeError::TypeMismatch(
                                    "Both operands of && must be boolean".to_string(),
                                )),
                            }
                        }
                    }
                    BinaryOp::Or => {
                        let l_val = self.evaluate_expression(left, env.clone())?;
                        if let Value::Boolean(true) = l_val {
                            // If left is true, return true without evaluating right (short-circuit)
                            Ok(Value::Boolean(true))
                        } else {
                            // If left is false, evaluate and return the right operand
                            let r_val = self.evaluate_expression(right, env.clone())?;
                            match (l_val, r_val) {
                                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l || r)),
                                _ => Err(RuntimeError::TypeMismatch(
                                    "Both operands of || must be boolean".to_string(),
                                )),
                            }
                        }
                    }
                    // For all other operators, evaluate both operands normally
                    _ => {
                        let l = self.evaluate_expression(left, env.clone())?;
                        let r = self.evaluate_expression(right, env.clone())?;
                        self.evaluate_binary(l, operator, r)
                    }
                }
            }
            ExpressionKind::Unary { operator, operand } => {
                let val = self.evaluate_expression(operand, env)?;
                match (operator, val) {
                    (UnaryOp::Minus, Value::Integer(i)) => Ok(Value::Integer(-i)),
                    (UnaryOp::Minus, Value::Float(f)) => Ok(Value::Float(-f)),
                    (UnaryOp::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
                    // Prefix increment and decrement (for now, treating as postfix since we handle both the same way)
                    (UnaryOp::Increment, Value::Integer(i)) => Ok(Value::Integer(i + 1)),
                    (UnaryOp::Decrement, Value::Integer(i)) => Ok(Value::Integer(i - 1)),
                    (UnaryOp::Increment, Value::Float(f)) => Ok(Value::Float(f + 1.0)),
                    (UnaryOp::Decrement, Value::Float(f)) => Ok(Value::Float(f - 1.0)),
                    _ => Err(RuntimeError::TypeMismatch(
                        "Invalid operand for unary operator".to_string(),
                    )),
                }
            }
            ExpressionKind::MemberAccess { object, member } => {
                let obj_val = self.evaluate_expression(object, env)?;
                match (obj_val, member.as_str()) {
                    (Value::String(s), "length") => Ok(Value::Integer(s.len() as i64)),
                    // Type conversion methods
                    (Value::String(s), "toInt") => {
                        match s.parse::<i64>() {
                            Ok(num) => Ok(Value::Integer(num)),
                            Err(_) => Err(RuntimeError::TypeMismatch(
                                format!("Cannot convert string '{}' to integer", s)
                            )),
                        }
                    },
                    (Value::String(s), "toFloat") => {
                        match s.parse::<f64>() {
                            Ok(num) => Ok(Value::Float(num)),
                            Err(_) => Err(RuntimeError::TypeMismatch(
                                format!("Cannot convert string '{}' to float", s)
                            )),
                        }
                    },
                    (Value::Integer(n), "toFloat") => Ok(Value::Float(n as f64)),
                    (Value::Float(f), "toInt") => Ok(Value::Integer(f as i64)),
                    (Value::Integer(n), "toString") => Ok(Value::String(n.to_string())),
                    (Value::Float(f), "toString") => Ok(Value::String(f.to_string())),
                    (Value::Boolean(b), "toString") => Ok(Value::String(b.to_string())),
                    (Value::Char(c), "toString") => Ok(Value::String(c.to_string())),
                    // Array methods
                    (Value::Array(_elements), "push") => {
                        // For member access, we don't have arguments, so this would be an error
                        Err(RuntimeError::TypeMismatch(
                            "Cannot call push() without arguments via member access".to_string(),
                        ))
                    },
                    (Value::Array(_elements), "pop") => {
                        // For member access, this would be an error since pop() should be callable
                        Err(RuntimeError::TypeMismatch(
                            "Cannot call pop() via member access".to_string(),
                        ))
                    },
                    // HashMap iteration methods
                    (Value::HashMap(map), "keys") => {
                        let keys: Vec<Value> = map.keys().map(|k| Value::String(k.clone())).collect();
                        Ok(Value::Array(keys))
                    },
                    (Value::HashMap(map), "iter") => {
                        // Create iterator over entries
                        let mut entries = Vec::new();
                        for (key, value) in map.iter() {
                            let entry = Value::Array(vec![Value::String(key.clone()), value.clone()]);
                            entries.push(entry);
                        }
                        let it = IteratorState { items: entries, pos: 0 };
                        Ok(Value::Iterator(Rc::new(RefCell::new(it))))
                    },
                    (Value::HashMap(map), "values") => {
                        let values: Vec<Value> = map.values().cloned().collect();
                        Ok(Value::Array(values))
                    },
                    (Value::HashMap(map), "size") => {
                        Ok(Value::Integer(map.len() as i64))
                    },
                    (Value::HashMap(map), "entries") => {
                        let mut entries = Vec::new();
                        for (key, value) in map.iter() {
                            // Create an array with [key, value] for each entry
                            let entry = Value::Array(vec![Value::String(key.clone()), value.clone()]);
                            entries.push(entry);
                        }
                        Ok(Value::Array(entries))
                    },
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
                    (Value::String(s), Value::Integer(index)) => {
                        let idx = index as usize;
                        if idx < s.len() {
                            // Return the character at the index as a Char value
                            let ch = s.chars().nth(idx).unwrap();
                            Ok(Value::Char(ch))
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                "String index out of bounds".to_string(),
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
                            "Indexing target is not an array or string".to_string(),
                        ))
                    }
                    (_, Value::String(_)) => {
                        Err(RuntimeError::TypeMismatch(
                            "Indexing target is not a HashMap".to_string(),
                        ))
                    }
                    _ => Err(RuntimeError::TypeMismatch(
                        "Index must be an integer for arrays/strings or string for HashMaps".to_string(),
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
                    Err(RuntimeError::Return(val)) => Ok(*val),
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
            
            // Float operations
            (Value::Float(l), BinaryOp::Add, Value::Float(r)) => Ok(Value::Float(l + r)),
            (Value::Float(l), BinaryOp::Sub, Value::Float(r)) => Ok(Value::Float(l - r)),
            (Value::Float(l), BinaryOp::Mul, Value::Float(r)) => Ok(Value::Float(l * r)),
            (Value::Float(l), BinaryOp::Div, Value::Float(r)) => Ok(Value::Float(l / r)),
            
            // Compound assignment operators for integers
            (Value::Integer(l), BinaryOp::PlusEqual, Value::Integer(r)) => Ok(Value::Integer(l + r)),
            (Value::Integer(l), BinaryOp::MinusEqual, Value::Integer(r)) => Ok(Value::Integer(l - r)),
            (Value::Integer(l), BinaryOp::StarEqual, Value::Integer(r)) => Ok(Value::Integer(l * r)),
            (Value::Integer(l), BinaryOp::SlashEqual, Value::Integer(r)) => Ok(Value::Integer(l / r)),
            
            // Compound assignment operators for floats
            (Value::Float(l), BinaryOp::PlusEqual, Value::Float(r)) => Ok(Value::Float(l + r)),
            (Value::Float(l), BinaryOp::MinusEqual, Value::Float(r)) => Ok(Value::Float(l - r)),
            (Value::Float(l), BinaryOp::StarEqual, Value::Float(r)) => Ok(Value::Float(l * r)),
            (Value::Float(l), BinaryOp::SlashEqual, Value::Float(r)) => Ok(Value::Float(l / r)),
            
            // Boolean operations
            (Value::Boolean(l), BinaryOp::Equal, Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
            (Value::Boolean(l), BinaryOp::NotEqual, Value::Boolean(r)) => {
                Ok(Value::Boolean(l != r))
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
            
            // Mixed operations for compound assignments
            (Value::Integer(l), BinaryOp::PlusEqual, Value::Float(r)) => Ok(Value::Float(l as f64 + r)),
            (Value::Float(l), BinaryOp::PlusEqual, Value::Integer(r)) => Ok(Value::Float(l + r as f64)),
            (Value::Integer(l), BinaryOp::MinusEqual, Value::Float(r)) => Ok(Value::Float(l as f64 - r)),
            (Value::Float(l), BinaryOp::MinusEqual, Value::Integer(r)) => Ok(Value::Float(l - r as f64)),
            (Value::Integer(l), BinaryOp::StarEqual, Value::Float(r)) => Ok(Value::Float(l as f64 * r)),
            (Value::Float(l), BinaryOp::StarEqual, Value::Integer(r)) => Ok(Value::Float(l * r as f64)),
            (Value::Integer(l), BinaryOp::SlashEqual, Value::Float(r)) => Ok(Value::Float(l as f64 / r)),
            (Value::Float(l), BinaryOp::SlashEqual, Value::Integer(r)) => Ok(Value::Float(l / r as f64)),
            
            // String concatenation for PlusEqual
            (Value::String(l), BinaryOp::PlusEqual, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::String(l), BinaryOp::PlusEqual, Value::Integer(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::Integer(l), BinaryOp::PlusEqual, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::String(l), BinaryOp::PlusEqual, Value::Float(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::Float(l), BinaryOp::PlusEqual, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::String(l), BinaryOp::PlusEqual, Value::Boolean(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::Boolean(l), BinaryOp::PlusEqual, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            _ => Err(RuntimeError::TypeMismatch(
                "Binary operator operand mismatch".to_string(),
            )),
            
        }
    }
}
