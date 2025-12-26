use dotlin_ast::*;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum TypeError {
    #[error("Type mismatch: expected {expected:?}, found {found:?}")]
    Mismatch { expected: Type, found: Type },
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
    #[error("Not a function: {0}")]
    NotAFunction(String),
    #[error("Incorrect argument count for function {name}: expected {expected}, got {got}")]
    ArgumentCount {
        name: String,
        expected: usize,
        got: usize,
    },
    #[error("Undefined member '{member}' on type {typ:?}")]
    UndefinedMember { typ: Type, member: String },
}

pub struct TypeChecker {
    scopes: Vec<HashMap<String, Type>>,
    functions: HashMap<String, (Vec<Type>, Option<Type>)>,
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        // Built-ins
        functions.insert("println".to_string(), (vec![], None)); // Special handling

        // Math functions
        functions.insert(
            "abs".to_string(),
            (
                vec![Type::Named("Float".to_string())],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "min".to_string(),
            (
                vec![
                    Type::Named("Float".to_string()),
                    Type::Named("Float".to_string()),
                ],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "max".to_string(),
            (
                vec![
                    Type::Named("Float".to_string()),
                    Type::Named("Float".to_string()),
                ],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "sqrt".to_string(),
            (
                vec![Type::Named("Float".to_string())],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "pow".to_string(),
            (
                vec![
                    Type::Named("Float".to_string()),
                    Type::Named("Float".to_string()),
                ],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "sin".to_string(),
            (
                vec![Type::Named("Float".to_string())],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "cos".to_string(),
            (
                vec![Type::Named("Float".to_string())],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "tan".to_string(),
            (
                vec![Type::Named("Float".to_string())],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "floor".to_string(),
            (
                vec![Type::Named("Float".to_string())],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "ceil".to_string(),
            (
                vec![Type::Named("Float".to_string())],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "round".to_string(),
            (
                vec![Type::Named("Float".to_string())],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "log".to_string(),
            (
                vec![Type::Named("Float".to_string())],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "exp".to_string(),
            (
                vec![Type::Named("Float".to_string())],
                Some(Type::Named("Float".to_string())),
            ),
        );
        functions.insert(
            "PI".to_string(),
            (vec![], Some(Type::Named("Float".to_string()))),
        );
        functions.insert(
            "E".to_string(),
            (vec![], Some(Type::Named("Float".to_string()))),
        );

        Self {
            scopes: vec![HashMap::new()],
            functions,
        }
    }

    pub fn check_program(&mut self, program: &mut Program) -> Result<(), TypeError> {
        // First pass: gather function signatures
        for decl in &program.declarations {
            let Declaration::Function(func) = decl;
            let params = func.params.iter().map(|p| p.typ.clone()).collect();
            self.functions
                .insert(func.name.clone(), (params, func.return_type.clone()));
        }

        // Second pass: check bodies
        for decl in &mut program.declarations {
            self.check_declaration(decl)?;
        }
        Ok(())
    }

    fn check_declaration(&mut self, decl: &mut Declaration) -> Result<(), TypeError> {
        match decl {
            Declaration::Function(func) => {
                self.scopes.push(HashMap::new());
                for param in &func.params {
                    self.define_var(param.name.clone(), param.typ.clone());
                }
                self.check_block(&mut func.body)?;
                self.scopes.pop();
                Ok(())
            }
        }
    }

    fn check_block(&mut self, block: &mut Block) -> Result<(), TypeError> {
        for stmt in &mut block.statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &mut Statement) -> Result<(), TypeError> {
        match stmt {
            Statement::Expression(expr) => {
                self.check_expression(expr)?;
                Ok(())
            }
            Statement::VariableDecl {
                name,
                typ,
                initializer,
            } => {
                let resolved_typ = if let Some(init) = initializer {
                    self.check_expression(init)?
                } else {
                    typ.clone().unwrap_or(Type::Named("Int".to_string()))
                };

                if let Some(explicit_typ) = typ {
                    if explicit_typ != &resolved_typ {
                        return Err(TypeError::Mismatch {
                            expected: explicit_typ.clone(),
                            found: resolved_typ,
                        });
                    }
                }

                *typ = Some(resolved_typ.clone());
                self.define_var(name.clone(), resolved_typ);
                Ok(())
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    self.check_expression(e)?;
                }
                Ok(())
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_typ = self.check_expression(condition)?;
                if cond_typ != Type::Named("Boolean".to_string()) {
                    return Err(TypeError::Mismatch {
                        expected: Type::Named("Boolean".to_string()),
                        found: cond_typ,
                    });
                }
                self.check_statement(then_branch)?;
                if let Some(els) = else_branch {
                    self.check_statement(els)?;
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                let cond_typ = self.check_expression(condition)?;
                if cond_typ != Type::Named("Boolean".to_string()) {
                    return Err(TypeError::Mismatch {
                        expected: Type::Named("Boolean".to_string()),
                        found: cond_typ,
                    });
                }
                self.check_statement(body)?;
                Ok(())
            }
            Statement::Block(block) => {
                self.scopes.push(HashMap::new());
                self.check_block(block)?;
                self.scopes.pop();
                Ok(())
            }
            Statement::ForEach {
                variable,
                iterable,
                body,
            } => {
                // If this is a call like `obj.iter()` and `obj` is a Map, allow tuple destructuring
                let mut handled = false;
                if let ExpressionKind::Call { callee, .. } = &mut *iterable.kind {
                    if let ExpressionKind::MemberAccess { object, member } = &*callee.kind {
                        if member == "iter" {
                            // Check the object type
                            let mut obj_expr = (*object).clone();
                            let obj_typ = self.check_expression(&mut obj_expr)?;
                            if let Type::Map(key_type, value_type) = obj_typ {
                                // Create a new scope for the for-each loop
                                self.scopes.push(HashMap::new());

                                match variable {
                                    dotlin_ast::ForEachTarget::Ident(name) => {
                                        // Iterating over a map with a single identifier yields keys
                                        self.define_var(name.clone(), (*key_type).clone());
                                    }
                                    dotlin_ast::ForEachTarget::Tuple(names) => {
                                        // If two names, assign key and value types respectively
                                        if names.len() == 2 {
                                            self.define_var(names[0].clone(), (*key_type).clone());
                                            self.define_var(
                                                names[1].clone(),
                                                (*value_type).clone(),
                                            );
                                        } else {
                                            // Fallback: assign key type to all names
                                            for n in names {
                                                self.define_var(n.clone(), (*key_type).clone());
                                            }
                                        }
                                    }
                                }

                                // Check the loop body
                                self.check_statement(body)?;

                                // Pop the scope
                                self.scopes.pop();
                                handled = true;
                            }
                        }
                    }
                }

                if handled {
                    return Ok(());
                }

                // Fallback: Check the iterable expression normally
                let iterable_type = self.check_expression(iterable)?;

                // For now, we'll handle iteration over arrays and HashMaps
                let element_type = match iterable_type {
                    Type::Array(element_type) => *element_type,
                    Type::Map(key_type, _) => {
                        // For HashMap iteration, we can iterate over keys, values, or entries
                        // For now, assume iterating over keys
                        *key_type
                    }
                    _ => {
                        return Err(TypeError::Mismatch {
                            expected: Type::Array(Box::new(Type::Named("Int".to_string()))),
                            found: iterable_type,
                        })
                    }
                };

                // Create a new scope for the for-each loop
                self.scopes.push(HashMap::new());

                // Add the loop variable(s) to the scope
                match variable {
                    dotlin_ast::ForEachTarget::Ident(name) => {
                        self.define_var(name.clone(), element_type.clone());
                    }
                    dotlin_ast::ForEachTarget::Tuple(names) => {
                        // For now, assign the same element_type to each destructured name
                        for n in names {
                            self.define_var(n.clone(), element_type.clone());
                        }
                    }
                }

                // Check the loop body
                self.check_statement(body)?;

                // Pop the scope
                self.scopes.pop();

                Ok(())
            }
        }
    }

    fn check_expression(&mut self, expr: &mut Expression) -> Result<Type, TypeError> {
        let typ = match &mut *expr.kind {
            ExpressionKind::Literal(lit) => match lit {
                Literal::Integer(_) => Type::Named("Int".to_string()),
                Literal::Float(_) => Type::Named("Float".to_string()),
                Literal::String(_) => Type::Named("String".to_string()),
                Literal::Boolean(_) => Type::Named("Boolean".to_string()),
                Literal::Char(_) => Type::Named("Char".to_string()),
            },
            ExpressionKind::Variable(name) => self.lookup_var(name)?.clone(),
            ExpressionKind::Assignment { name, value } => {
                let var_typ = self.lookup_var(name)?.clone();
                let val_typ = self.check_expression(value)?;
                if var_typ != val_typ {
                    return Err(TypeError::Mismatch {
                        expected: var_typ,
                        found: val_typ,
                    });
                }
                val_typ
            }
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                let lt = self.check_expression(left)?;
                let rt = self.check_expression(right)?;

                match operator {
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::PlusEqual
                    | BinaryOp::MinusEqual
                    | BinaryOp::StarEqual
                    | BinaryOp::SlashEqual => {
                        if lt == Type::Named("String".to_string())
                            && rt == Type::Named("String".to_string())
                            && matches!(operator, BinaryOp::Add)
                        {
                            Type::Named("String".to_string())
                        } else if lt == rt
                            && (lt == Type::Named("Int".to_string())
                                || lt == Type::Named("Float".to_string()))
                        {
                            lt
                        } else {
                            return Err(TypeError::Mismatch {
                                expected: lt,
                                found: rt,
                            });
                        }
                    }
                    BinaryOp::Equal
                    | BinaryOp::NotEqual
                    | BinaryOp::Less
                    | BinaryOp::LessEqual
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEqual => {
                        if lt != rt {
                            return Err(TypeError::Mismatch {
                                expected: lt,
                                found: rt,
                            });
                        }
                        Type::Named("Boolean".to_string())
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if lt != Type::Named("Boolean".to_string()) {
                            return Err(TypeError::Mismatch {
                                expected: Type::Named("Boolean".to_string()),
                                found: lt,
                            });
                        }
                        if rt != Type::Named("Boolean".to_string()) {
                            return Err(TypeError::Mismatch {
                                expected: Type::Named("Boolean".to_string()),
                                found: rt,
                            });
                        }
                        Type::Named("Boolean".to_string())
                    }
                }
            }
            ExpressionKind::Unary { operator, operand } => {
                let ot = self.check_expression(operand)?;
                match operator {
                    UnaryOp::Minus => {
                        if ot != Type::Named("Int".to_string())
                            && ot != Type::Named("Float".to_string())
                        {
                            return Err(TypeError::Mismatch {
                                expected: Type::Named("Int".to_string()),
                                found: ot,
                            });
                        }
                        ot
                    }
                    UnaryOp::Not => {
                        if ot != Type::Named("Boolean".to_string()) {
                            return Err(TypeError::Mismatch {
                                expected: Type::Named("Boolean".to_string()),
                                found: ot,
                            });
                        }
                        ot
                    }
                    UnaryOp::Increment | UnaryOp::Decrement => {
                        // Increment and decrement can only be applied to numeric types
                        if ot != Type::Named("Int".to_string())
                            && ot != Type::Named("Float".to_string())
                        {
                            return Err(TypeError::Mismatch {
                                expected: Type::Named("Int".to_string()),
                                found: ot,
                            });
                        }
                        ot // The result type is the same as the operand type
                    }
                }
            }
            ExpressionKind::Call { callee, arguments } => {
                // Handle method calls (obj.method()) which are represented as Call with MemberAccess callee
                if let ExpressionKind::MemberAccess { object, member } = &*callee.kind {
                    // This is a method call on an object
                    // We need to handle mutability properly
                    let mut obj_expr = (*object).clone();
                    let obj_typ = self.check_expression(&mut obj_expr)?;

                    // Handle type conversion methods and HashMap iteration methods
                    match (&obj_typ, member.as_str()) {
                        // String conversion methods
                        (Type::Named(name), "toInt") if name == "String" => {
                            Type::Named("Int".to_string())
                        }
                        (Type::Named(name), "toFloat") if name == "String" => {
                            Type::Named("Float".to_string())
                        }

                        // Numeric conversion methods
                        (Type::Named(name), "toFloat") if name == "Int" => {
                            Type::Named("Float".to_string())
                        }
                        (Type::Named(name), "toInt") if name == "Float" => {
                            Type::Named("Int".to_string())
                        }

                        // To string methods
                        (Type::Named(name), "toString") if name == "Int" => {
                            Type::Named("String".to_string())
                        }
                        (Type::Named(name), "toString") if name == "Float" => {
                            Type::Named("String".to_string())
                        }
                        (Type::Named(name), "toString") if name == "Boolean" => {
                            Type::Named("String".to_string())
                        }
                        (Type::Named(name), "toString") if name == "Char" => {
                            Type::Named("String".to_string())
                        }

                        // Array methods
                        (Type::Array(_), "push") => Type::Named("Int".to_string()), // returns void but using Int as placeholder
                        (Type::Array(_), "pop") => Type::Named("Int".to_string()), // returns the popped element

                        // HashMap iteration methods
                        (Type::Map(_, _), "keys") => {
                            Type::Array(Box::new(Type::Named("String".to_string())))
                        } // Returns array of keys
                        (Type::Map(_, _), "values") => {
                            Type::Array(Box::new(Type::Named("Int".to_string())))
                        } // Returns array of values (for now)
                        (Type::Map(_, _), "size") => Type::Named("Int".to_string()), // Returns size as int
                        (Type::Map(_, _), "entries") => {
                            Type::Array(Box::new(Type::Named("Int".to_string())))
                        } // Returns array of alternating key-value pairs (for now)

                        // Undefined method
                        (obj_type, method_name) => {
                            return Err(TypeError::UndefinedMember {
                                typ: obj_type.clone(),
                                member: method_name.to_string(),
                            })
                        }
                    }
                } else if let ExpressionKind::Variable(name) = &*callee.kind {
                    // Regular function call
                    if name == "println" {
                        // println is special, accepts anything for now
                        for arg in arguments {
                            self.check_expression(arg)?;
                        }
                        Type::Named("Int".to_string())
                    } else if let Some((params, ret)) = self.functions.get(name).cloned() {
                        if params.len() != arguments.len() {
                            return Err(TypeError::ArgumentCount {
                                name: name.clone(),
                                expected: params.len(),
                                got: arguments.len(),
                            });
                        }
                        for (i, arg) in arguments.iter_mut().enumerate() {
                            let at = self.check_expression(arg)?;
                            if at != params[i] {
                                return Err(TypeError::Mismatch {
                                    expected: params[i].clone(),
                                    found: at,
                                });
                            }
                        }
                        ret.unwrap_or(Type::Named("Int".to_string()))
                    } else {
                        return Err(TypeError::UndefinedVariable(name.clone()));
                    }
                } else {
                    return Err(TypeError::NotAFunction(
                        "Expression is not a variable".to_string(),
                    ));
                }
            }
            ExpressionKind::MemberAccess { object, member } => {
                let obj_typ = self.check_expression(object)?;
                match (&obj_typ, member.as_str()) {
                    // String length property
                    (Type::Named(name), "length") if name == "String" => {
                        Type::Named("Int".to_string())
                    }

                    // Type conversion methods
                    (Type::Named(name), "toInt") if name == "String" => {
                        Type::Named("Int".to_string())
                    }
                    (Type::Named(name), "toFloat") if name == "String" => {
                        Type::Named("Float".to_string())
                    }
                    (Type::Named(name), "toFloat") if name == "Int" => {
                        Type::Named("Float".to_string())
                    }
                    (Type::Named(name), "toInt") if name == "Float" => {
                        Type::Named("Int".to_string())
                    }
                    (Type::Named(name), "toString") if name == "Int" => {
                        Type::Named("String".to_string())
                    }
                    (Type::Named(name), "toString") if name == "Float" => {
                        Type::Named("String".to_string())
                    }
                    (Type::Named(name), "toString") if name == "Boolean" => {
                        Type::Named("String".to_string())
                    }
                    (Type::Named(name), "toString") if name == "Char" => {
                        Type::Named("String".to_string())
                    }

                    // Array methods
                    (Type::Array(_), "push") => Type::Named("Int".to_string()), // returns void but using Int as placeholder
                    (Type::Array(_), "pop") => Type::Named("Int".to_string()), // returns the popped value

                    // HashMap iteration methods
                    (Type::Map(_, _), "keys") => {
                        Type::Array(Box::new(Type::Named("String".to_string())))
                    } // Returns array of keys
                    (Type::Map(_, _), "values") => {
                        Type::Array(Box::new(Type::Named("Int".to_string())))
                    } // Returns array of values (for now)
                    (Type::Map(_, _), "size") => Type::Named("Int".to_string()), // Returns size as int
                    (Type::Map(_, _), "entries") => {
                        Type::Array(Box::new(Type::Named("Int".to_string())))
                    } // Returns array of alternating key-value pairs (for now)

                    // Undefined member access
                    _ => {
                        return Err(TypeError::UndefinedMember {
                            typ: obj_typ,
                            member: member.clone(),
                        })
                    }
                }
            }
            ExpressionKind::ArrayLiteral { elements } => {
                if elements.is_empty() {
                    // Default to Int array for empty arrays
                    Type::Array(Box::new(Type::Named("Int".to_string())))
                } else {
                    let first_elem_type = self.check_expression(&mut elements[0])?;
                    for element in elements.iter_mut().skip(1) {
                        let elem_type = self.check_expression(element)?;
                        if elem_type != first_elem_type {
                            return Err(TypeError::Mismatch {
                                expected: first_elem_type.clone(),
                                found: elem_type,
                            });
                        }
                    }
                    Type::Array(Box::new(first_elem_type))
                }
            }
            ExpressionKind::Index { array, index } => {
                let arr_typ = self.check_expression(array)?;
                let idx_typ = self.check_expression(index)?;

                match &arr_typ {
                    Type::Array(element_type) => {
                        // Array indexing: index must be Int
                        if idx_typ != Type::Named("Int".to_string()) {
                            return Err(TypeError::Mismatch {
                                expected: Type::Named("Int".to_string()),
                                found: idx_typ,
                            });
                        }
                        *element_type.clone()
                    }
                    Type::Named(name) if name == "String" => {
                        // String indexing: index must be Int
                        if idx_typ != Type::Named("Int".to_string()) {
                            return Err(TypeError::Mismatch {
                                expected: Type::Named("Int".to_string()),
                                found: idx_typ,
                            });
                        }
                        Type::Named("Char".to_string()) // String indexing returns Char
                    }
                    Type::Map(key_type, value_type) => {
                        // HashMap indexing: key type must match map's key type
                        if idx_typ != **key_type {
                            return Err(TypeError::Mismatch {
                                expected: key_type.as_ref().clone(),
                                found: idx_typ,
                            });
                        }
                        *value_type.clone()
                    }
                    _ => {
                        return Err(TypeError::UndefinedMember {
                            typ: arr_typ,
                            member: "index".to_string(),
                        });
                    }
                }
            }
            ExpressionKind::HashMapLiteral { pairs } => {
                if pairs.is_empty() {
                    // Default to Map<String, Int> for empty HashMap literals
                    Type::Map(
                        Box::new(Type::Named("String".to_string())),
                        Box::new(Type::Named("Int".to_string())),
                    )
                } else {
                    // Determine key and value types from the first pair
                    let (first_key, first_val) = &mut pairs[0];
                    let key_type = self.check_expression(first_key)?;
                    let value_type = self.check_expression(first_val)?;

                    // Check that all other pairs have consistent types
                    for (key, val) in pairs.iter_mut().skip(1) {
                        let k_type = self.check_expression(key)?;
                        let v_type = self.check_expression(val)?;

                        if k_type != key_type {
                            return Err(TypeError::Mismatch {
                                expected: key_type.clone(),
                                found: k_type,
                            });
                        }
                        if v_type != value_type {
                            return Err(TypeError::Mismatch {
                                expected: value_type.clone(),
                                found: v_type,
                            });
                        }
                    }

                    Type::Map(Box::new(key_type), Box::new(value_type))
                }
            }
        };
        expr.resolved_type = Some(typ.clone());
        Ok(typ)
    }

    fn define_var(&mut self, name: String, typ: Type) {
        self.scopes.last_mut().unwrap().insert(name, typ);
    }

    fn lookup_var(&self, name: &str) -> Result<&Type, TypeError> {
        for scope in self.scopes.iter().rev() {
            if let Some(typ) = scope.get(name) {
                return Ok(typ);
            }
        }
        Err(TypeError::UndefinedVariable(name.to_string()))
    }
}
