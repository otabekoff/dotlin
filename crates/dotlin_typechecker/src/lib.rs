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

impl TypeChecker {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        // Built-ins
        functions.insert("println".to_string(), (vec![], None)); // Special handling

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
        }
    }

    fn check_expression(&mut self, expr: &mut Expression) -> Result<Type, TypeError> {
        let typ = match &mut *expr.kind {
            ExpressionKind::Literal(lit) => match lit {
                Literal::Integer(_) => Type::Named("Int".to_string()),
                Literal::Float(_) => Type::Named("Float".to_string()),
                Literal::String(_) => Type::Named("String".to_string()),
                Literal::Boolean(_) => Type::Named("Boolean".to_string()),
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
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
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
                }
            }
            ExpressionKind::Call { callee, arguments } => {
                if let ExpressionKind::Variable(ref name) = &*callee.kind {
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
                if obj_typ == Type::Named("String".to_string()) && member == "length" {
                    Type::Named("Int".to_string())
                } else {
                    return Err(TypeError::UndefinedMember {
                        typ: obj_typ,
                        member: member.clone(),
                    });
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
