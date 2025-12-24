use cranelift_codegen::ir::{
    condcodes::{FloatCC, IntCC},
    types, AbiParam, InstBuilder, MemFlags, Signature, Value,
};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{DataDescription, DataId, FuncId, Linkage, Module, ModuleError};
use cranelift_native;
use cranelift_object::{ObjectBuilder, ObjectModule};
use dotlin_ast::*;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum CompileError {
    #[error("Cranelift module error: {0}")]
    Module(#[from] ModuleError),
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DotlinType {
    Int,
    Float,
    Boolean,
    String,
    Array,
}

pub struct CodeGenerator {
    module: ObjectModule,
    functions: HashMap<String, (FuncId, Option<DotlinType>)>,
    strings: HashMap<String, DataId>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("opt_level", "speed").unwrap();
        let isa_builder = cranelift_native::builder()
            .unwrap()
            .finish(settings::Flags::new(flag_builder))
            .unwrap();

        let builder = ObjectBuilder::new(
            isa_builder,
            "dotlin_module",
            cranelift_module::default_libcall_names(),
        )
        .unwrap();
        let module = ObjectModule::new(builder);

        Self {
            module,
            functions: HashMap::new(),
            strings: HashMap::new(),
        }
    }

    fn dotlin_type(typ: &Type) -> DotlinType {
        match typ {
            Type::Named(name) => match name.as_str() {
                "Int" => DotlinType::Int,
                "Float" => DotlinType::Float,
                "Boolean" => DotlinType::Boolean,
                "String" => DotlinType::String,
                _ => DotlinType::Int,
            },
            Type::Array(_) => DotlinType::Array,
            Type::Map(_, _) => DotlinType::Int, // Maps are represented as pointers like other objects
            Type::Generic(_, _) => DotlinType::Int, // Generic types resolve to their concrete types
        }
    }

    fn cl_type(typ: &Type) -> cranelift_codegen::ir::Type {
        match Self::dotlin_type(typ) {
            DotlinType::Int => types::I64,
            DotlinType::Float => types::F64,
            DotlinType::Boolean => types::I8,
            DotlinType::String => types::I64,
            DotlinType::Array => types::I64, // Arrays are represented as pointers
        }
    }

    fn make_sig(&self, func: &FunctionDecl) -> Signature {
        let mut sig = self.module.make_signature();
        for param in &func.params {
            sig.params.push(AbiParam::new(Self::cl_type(&param.typ)));
        }
        if let Some(ref ret_type) = func.return_type {
            sig.returns.push(AbiParam::new(Self::cl_type(ret_type)));
        }
        sig
    }

    pub fn compile_program(mut self, program: &Program) -> Result<Vec<u8>, CompileError> {
        let mut sig_i64 = self.module.make_signature();
        sig_i64.params.push(AbiParam::new(types::I64));
        let println_i64 = self
            .module
            .declare_function("println_i64", Linkage::Import, &sig_i64)?;
        self.functions
            .insert("println_i64".to_string(), (println_i64, None));

        let mut sig_str = self.module.make_signature();
        sig_str.params.push(AbiParam::new(types::I64));
        let println_str = self
            .module
            .declare_function("println_str", Linkage::Import, &sig_str)?;
        self.functions
            .insert("println_str".to_string(), (println_str, None));

        let mut sig_concat = self.module.make_signature();
        sig_concat.params.push(AbiParam::new(types::I64));
        sig_concat.params.push(AbiParam::new(types::I64));
        sig_concat.returns.push(AbiParam::new(types::I64));
        let string_concat =
            self.module
                .declare_function("dotlin_string_concat", Linkage::Import, &sig_concat)?;
        self.functions.insert(
            "dotlin_string_concat".to_string(),
            (string_concat, Some(DotlinType::String)),
        );

        let mut sig_f64 = self.module.make_signature();
        sig_f64.params.push(AbiParam::new(types::F64));
        let println_f64 = self
            .module
            .declare_function("println_f64", Linkage::Import, &sig_f64)?;
        self.functions
            .insert("println_f64".to_string(), (println_f64, None));

        let mut sig_cmp = self.module.make_signature();
        sig_cmp.params.push(AbiParam::new(types::I64));
        sig_cmp.params.push(AbiParam::new(types::I64));
        sig_cmp.returns.push(AbiParam::new(types::I64));
        let string_compare =
            self.module
                .declare_function("dotlin_string_compare", Linkage::Import, &sig_cmp)?;
        self.functions.insert(
            "dotlin_string_compare".to_string(),
            (string_compare, Some(DotlinType::Int)),
        );
        
        // Array functions
        let mut sig_array_new = self.module.make_signature();
        sig_array_new.params.push(AbiParam::new(types::I64)); // element_size
        sig_array_new.params.push(AbiParam::new(types::I64)); // capacity
        sig_array_new.returns.push(AbiParam::new(types::I64));
        let array_new = self
            .module
            .declare_function("dotlin_array_new", Linkage::Import, &sig_array_new)?;
        self.functions
            .insert("dotlin_array_new".to_string(), (array_new, Some(DotlinType::Array)));
        
        let mut sig_array_get = self.module.make_signature();
        sig_array_get.params.push(AbiParam::new(types::I64)); // array_ptr
        sig_array_get.params.push(AbiParam::new(types::I64)); // index
        sig_array_get.returns.push(AbiParam::new(types::I64));
        let array_get = self
            .module
            .declare_function("dotlin_array_get", Linkage::Import, &sig_array_get)?;
        self.functions
            .insert("dotlin_array_get".to_string(), (array_get, Some(DotlinType::Int)));
        
        let mut sig_array_set = self.module.make_signature();
        sig_array_set.params.push(AbiParam::new(types::I64)); // array_ptr
        sig_array_set.params.push(AbiParam::new(types::I64)); // index
        sig_array_set.params.push(AbiParam::new(types::I64)); // value
        let array_set = self
            .module
            .declare_function("dotlin_array_set", Linkage::Import, &sig_array_set)?;
        self.functions
            .insert("dotlin_array_set".to_string(), (array_set, None));
        
        let mut sig_array_length = self.module.make_signature();
        sig_array_length.params.push(AbiParam::new(types::I64)); // array_ptr
        sig_array_length.returns.push(AbiParam::new(types::I64));
        let array_length = self
            .module
            .declare_function("dotlin_array_length", Linkage::Import, &sig_array_length)?;
        self.functions
            .insert("dotlin_array_length".to_string(), (array_length, Some(DotlinType::Int)));
        
        // HashMap functions
        let mut sig_map_new = self.module.make_signature();
        sig_map_new.returns.push(AbiParam::new(types::I64));
        let map_new = self
            .module
            .declare_function("dotlin_map_new", Linkage::Import, &sig_map_new)?;
        self.functions
            .insert("dotlin_map_new".to_string(), (map_new, Some(DotlinType::Int)));
        
        let mut sig_map_get = self.module.make_signature();
        sig_map_get.params.push(AbiParam::new(types::I64));
        sig_map_get.params.push(AbiParam::new(types::I64));
        sig_map_get.returns.push(AbiParam::new(types::I64));
        let map_get = self
            .module
            .declare_function("dotlin_map_get", Linkage::Import, &sig_map_get)?;
        self.functions
            .insert("dotlin_map_get".to_string(), (map_get, Some(DotlinType::Int)));
        
        let mut sig_map_set = self.module.make_signature();
        sig_map_set.params.push(AbiParam::new(types::I64));
        sig_map_set.params.push(AbiParam::new(types::I64));
        sig_map_set.params.push(AbiParam::new(types::I64));
        let map_set = self
            .module
            .declare_function("dotlin_map_set", Linkage::Import, &sig_map_set)?;
        self.functions
            .insert("dotlin_map_set".to_string(), (map_set, None));
        
        let mut sig_map_remove = self.module.make_signature();
        sig_map_remove.params.push(AbiParam::new(types::I64));
        sig_map_remove.params.push(AbiParam::new(types::I64));
        sig_map_remove.returns.push(AbiParam::new(types::I64));
        let map_remove = self
            .module
            .declare_function("dotlin_map_remove", Linkage::Import, &sig_map_remove)?;
        self.functions
            .insert("dotlin_map_remove".to_string(), (map_remove, Some(DotlinType::Int)));
        
        let mut sig_map_contains = self.module.make_signature();
        sig_map_contains.params.push(AbiParam::new(types::I64));
        sig_map_contains.params.push(AbiParam::new(types::I64));
        sig_map_contains.returns.push(AbiParam::new(types::I64));
        let map_contains = self
            .module
            .declare_function("dotlin_map_contains", Linkage::Import, &sig_map_contains)?;
        self.functions
            .insert("dotlin_map_contains".to_string(), (map_contains, Some(DotlinType::Int)));
        
        let mut sig_map_free = self.module.make_signature();
        sig_map_free.params.push(AbiParam::new(types::I64));
        let map_free = self
            .module
            .declare_function("dotlin_map_free", Linkage::Import, &sig_map_free)?;
        self.functions
            .insert("dotlin_map_free".to_string(), (map_free, None));
        
        // HashMap iteration functions
        let mut sig_map_keys = self.module.make_signature();
        sig_map_keys.params.push(AbiParam::new(types::I64)); // map_ptr
        sig_map_keys.returns.push(AbiParam::new(types::I64)); // array_ptr
        let map_keys = self
            .module
            .declare_function("dotlin_map_keys", Linkage::Import, &sig_map_keys)?;
        self.functions
            .insert("dotlin_map_keys".to_string(), (map_keys, Some(DotlinType::Array)));
        
        let mut sig_map_values = self.module.make_signature();
        sig_map_values.params.push(AbiParam::new(types::I64)); // map_ptr
        sig_map_values.returns.push(AbiParam::new(types::I64)); // array_ptr
        let map_values = self
            .module
            .declare_function("dotlin_map_values", Linkage::Import, &sig_map_values)?;
        self.functions
            .insert("dotlin_map_values".to_string(), (map_values, Some(DotlinType::Array)));
        
        let mut sig_map_size = self.module.make_signature();
        sig_map_size.params.push(AbiParam::new(types::I64)); // map_ptr
        sig_map_size.returns.push(AbiParam::new(types::I64)); // size
        let map_size = self
            .module
            .declare_function("dotlin_map_size", Linkage::Import, &sig_map_size)?;
        self.functions
            .insert("dotlin_map_size".to_string(), (map_size, Some(DotlinType::Int)));

        for decl in &program.declarations {
            let Declaration::Function(func) = decl;
            let sig = self.make_sig(func);
            let name = if func.name == "main" {
                "main_lin"
            } else {
                &func.name
            };
            let id = self.module.declare_function(name, Linkage::Export, &sig)?;
            let ret_type = func.return_type.as_ref().map(Self::dotlin_type);
            self.functions.insert(func.name.clone(), (id, ret_type));
        }

        for decl in &program.declarations {
            let Declaration::Function(func) = decl;
            let mut context = self.module.make_context();
            context.func.signature = self.make_sig(func);
            let mut func_ctx = FunctionBuilderContext::new();

            {
                let mut builder = FunctionBuilder::new(&mut context.func, &mut func_ctx);
                let block = builder.create_block();

                let mut param_vals = Vec::new();
                for param in &func.params {
                    param_vals.push(builder.append_block_param(block, Self::cl_type(&param.typ)));
                }

                builder.switch_to_block(block);

                let mut variables = HashMap::new();
                for (i, param) in func.params.iter().enumerate() {
                    let var = Variable::from_u32(i as u32);
                    let ty = Self::cl_type(&param.typ);
                    let dt = Self::dotlin_type(&param.typ);
                    builder.declare_var(var, ty);
                    builder.def_var(var, param_vals[i]);
                    variables.insert(param.name.clone(), (var, dt));
                }

                let mut var_idx = func.params.len() as u32;
                let mut terminated = false;
                for stmt in &func.body.statements {
                    terminated = Self::compile_statement(
                        &mut self.module,
                        &mut builder,
                        &mut self.strings,
                        &self.functions,
                        stmt,
                        &mut variables,
                        &mut var_idx,
                    )?;
                    if terminated {
                        break;
                    }
                }

                if !terminated {
                    if func.return_type.is_none() {
                        builder.ins().return_(&[]);
                    } else {
                        let zero = builder.ins().iconst(types::I64, 0);
                        builder.ins().return_(&[zero]);
                    }
                }
                builder.seal_all_blocks();
                builder.finalize();
            }

            let id = self.functions.get(&func.name).unwrap().0;
            self.module.define_function(id, &mut context)?;
        }

        let product = self.module.finish();
        Ok(product
            .emit()
            .map_err(|e| CompileError::Module(ModuleError::Backend(anyhow::anyhow!("{:?}", e))))?)
    }

    fn compile_statement(
        module: &mut ObjectModule,
        builder: &mut FunctionBuilder,
        strings: &mut HashMap<String, DataId>,
        functions: &HashMap<String, (FuncId, Option<DotlinType>)>,
        stmt: &Statement,
        vars: &mut HashMap<String, (Variable, DotlinType)>,
        var_index: &mut u32,
    ) -> Result<bool, CompileError> {
        match stmt {
            Statement::Expression(expr) => {
                Self::compile_expression(module, builder, strings, functions, expr, vars)?;
                Ok(false)
            }
            Statement::VariableDecl {
                name, initializer, ..
            } => {
                let (val, dt) = if let Some(init) = initializer {
                    Self::compile_expression(module, builder, strings, functions, init, vars)?
                } else {
                    (builder.ins().iconst(types::I64, 0), DotlinType::Int)
                };
                let var = Variable::from_u32(*var_index);
                *var_index += 1;
                vars.insert(name.clone(), (var, dt));
                let ty = builder.func.dfg.value_type(val);
                builder.declare_var(var, ty);
                builder.def_var(var, val);
                Ok(false)
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    let (val, _) =
                        Self::compile_expression(module, builder, strings, functions, e, vars)?;
                    builder.ins().return_(&[val]);
                } else {
                    builder.ins().return_(&[]);
                }
                Ok(true)
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let (cond, _) =
                    Self::compile_expression(module, builder, strings, functions, condition, vars)?;
                let then_block = builder.create_block();
                let else_block = builder.create_block();
                let merge_block = builder.create_block();

                builder.ins().brif(cond, then_block, &[], else_block, &[]);

                builder.switch_to_block(then_block);
                builder.seal_block(then_block);
                let then_terminated = Self::compile_statement(
                    module,
                    builder,
                    strings,
                    functions,
                    then_branch,
                    vars,
                    var_index,
                )?;
                if !then_terminated {
                    builder.ins().jump(merge_block, &[]);
                }

                builder.switch_to_block(else_block);
                builder.seal_block(else_block);
                let else_terminated = if let Some(els) = else_branch {
                    Self::compile_statement(
                        module, builder, strings, functions, els, vars, var_index,
                    )?
                } else {
                    false
                };
                if !else_terminated {
                    builder.ins().jump(merge_block, &[]);
                }

                if then_terminated && else_terminated {
                    return Ok(true);
                }

                builder.switch_to_block(merge_block);
                builder.seal_block(merge_block);
                Ok(false)
            }
            Statement::While { condition, body } => {
                let header = builder.create_block();
                let body_block = builder.create_block();
                let exit = builder.create_block();

                builder.ins().jump(header, &[]);
                builder.switch_to_block(header);
                let (cond, _) =
                    Self::compile_expression(module, builder, strings, functions, condition, vars)?;
                builder.ins().brif(cond, body_block, &[], exit, &[]);

                builder.switch_to_block(body_block);
                builder.seal_block(body_block);
                let body_terminated = Self::compile_statement(
                    module, builder, strings, functions, body, vars, var_index,
                )?;
                if !body_terminated {
                    builder.ins().jump(header, &[]);
                }
                builder.seal_block(header);

                builder.switch_to_block(exit);
                builder.seal_block(exit);
                Ok(false)
            }
            Statement::Block(block) => {
                let mut terminated = false;
                for s in &block.statements {
                    terminated = Self::compile_statement(
                        module, builder, strings, functions, s, vars, var_index,
                    )?;
                    if terminated {
                        break;
                    }
                }
                Ok(terminated)
            }
        }
    }

    fn compile_expression(
        module: &mut ObjectModule,
        builder: &mut FunctionBuilder,
        strings: &mut HashMap<String, DataId>,
        functions: &HashMap<String, (FuncId, Option<DotlinType>)>,
        expr: &Expression,
        vars: &HashMap<String, (Variable, DotlinType)>,
    ) -> Result<(Value, DotlinType), CompileError> {
        match &*expr.kind {
            ExpressionKind::Literal(lit) => match lit {
                Literal::Integer(i) => Ok((builder.ins().iconst(types::I64, *i), DotlinType::Int)),
                Literal::Float(f) => Ok((builder.ins().f64const(*f), DotlinType::Float)),
                Literal::Boolean(b) => Ok((
                    builder.ins().iconst(types::I8, if *b { 1 } else { 0 }),
                    DotlinType::Boolean,
                )),
                Literal::String(s) => {
                    let data_id = if let Some(id) = strings.get(s) {
                        *id
                    } else {
                        let mut desc = DataDescription::new();
                        desc.set_align(8);
                        let mut bytes = (s.len() as u64).to_le_bytes().to_vec();
                        bytes.extend_from_slice(s.as_bytes());
                        desc.define(bytes.into_boxed_slice());
                        let id = module
                            .declare_data(
                                &format!("str_{}", strings.len()),
                                Linkage::Local,
                                false,
                                false,
                            )
                            .map_err(CompileError::Module)?;
                        module
                            .define_data(id, &desc)
                            .map_err(CompileError::Module)?;
                        strings.insert(s.to_string(), id);
                        id
                    };
                    let global = module.declare_data_in_func(data_id, &mut builder.func);
                    Ok((
                        builder.ins().symbol_value(types::I64, global),
                        DotlinType::String,
                    ))
                }
            },
            ExpressionKind::Variable(name) => {
                if let Some((var, dt)) = vars.get(name) {
                    Ok((builder.use_var(*var), *dt))
                } else {
                    Err(CompileError::UndefinedVariable(name.clone()))
                }
            }
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                let (l, lt) =
                    Self::compile_expression(module, builder, strings, functions, left, vars)?;
                let (r, rt) =
                    Self::compile_expression(module, builder, strings, functions, right, vars)?;

                if lt == DotlinType::String && rt == DotlinType::String {
                    if matches!(operator, BinaryOp::Add) {
                        let (func_id, _) = functions.get("dotlin_string_concat").unwrap();
                        let func_ref = module.declare_func_in_func(*func_id, &mut builder.func);
                        let call = builder.ins().call(func_ref, &[l, r]);
                        let res = builder.inst_results(call)[0];
                        return Ok((res, DotlinType::String));
                    } else {
                        let (func_id, _) = functions.get("dotlin_string_compare").unwrap();
                        let func_ref = module.declare_func_in_func(*func_id, &mut builder.func);
                        let call = builder.ins().call(func_ref, &[l, r]);
                        let res = builder.inst_results(call)[0];
                        let zero = builder.ins().iconst(types::I64, 0);
                        let cond = match operator {
                            BinaryOp::Equal => IntCC::Equal,
                            BinaryOp::NotEqual => IntCC::NotEqual,
                            BinaryOp::Less => IntCC::SignedLessThan,
                            BinaryOp::LessEqual => IntCC::SignedLessThanOrEqual,
                            BinaryOp::Greater => IntCC::SignedGreaterThan,
                            BinaryOp::GreaterEqual => IntCC::SignedGreaterThanOrEqual,
                            _ => IntCC::Equal,
                        };
                        let cmp_val = builder.ins().icmp(cond, res, zero);
                        return Ok((cmp_val, DotlinType::Boolean));
                    }
                }

                if lt == DotlinType::Float || rt == DotlinType::Float {
                    let res = match operator {
                        BinaryOp::Add => builder.ins().fadd(l, r),
                        BinaryOp::Sub => builder.ins().fsub(l, r),
                        BinaryOp::Mul => builder.ins().fmul(l, r),
                        BinaryOp::Div => builder.ins().fdiv(l, r),
                        BinaryOp::Equal => builder.ins().fcmp(FloatCC::Equal, l, r),
                        BinaryOp::NotEqual => builder.ins().fcmp(FloatCC::NotEqual, l, r),
                        BinaryOp::Less => builder.ins().fcmp(FloatCC::LessThan, l, r),
                        BinaryOp::LessEqual => builder.ins().fcmp(FloatCC::LessThanOrEqual, l, r),
                        BinaryOp::Greater => builder.ins().fcmp(FloatCC::GreaterThan, l, r),
                        BinaryOp::GreaterEqual => {
                            builder.ins().fcmp(FloatCC::GreaterThanOrEqual, l, r)
                        }
                    };
                    let out_dt = match operator {
                        BinaryOp::Equal
                        | BinaryOp::NotEqual
                        | BinaryOp::Less
                        | BinaryOp::LessEqual
                        | BinaryOp::Greater
                        | BinaryOp::GreaterEqual => DotlinType::Boolean,
                        _ => DotlinType::Float,
                    };
                    Ok((res, out_dt))
                } else {
                    let res = match operator {
                        BinaryOp::Add => builder.ins().iadd(l, r),
                        BinaryOp::Sub => builder.ins().isub(l, r),
                        BinaryOp::Mul => builder.ins().imul(l, r),
                        BinaryOp::Div => builder.ins().sdiv(l, r),
                        BinaryOp::Equal => builder.ins().icmp(IntCC::Equal, l, r),
                        BinaryOp::NotEqual => builder.ins().icmp(IntCC::NotEqual, l, r),
                        BinaryOp::Less => builder.ins().icmp(IntCC::SignedLessThan, l, r),
                        BinaryOp::LessEqual => {
                            builder.ins().icmp(IntCC::SignedLessThanOrEqual, l, r)
                        }
                        BinaryOp::Greater => builder.ins().icmp(IntCC::SignedGreaterThan, l, r),
                        BinaryOp::GreaterEqual => {
                            builder.ins().icmp(IntCC::SignedGreaterThanOrEqual, l, r)
                        }
                    };
                    let out_dt = match operator {
                        BinaryOp::Equal
                        | BinaryOp::NotEqual
                        | BinaryOp::Less
                        | BinaryOp::LessEqual
                        | BinaryOp::Greater
                        | BinaryOp::GreaterEqual => DotlinType::Boolean,
                        _ => DotlinType::Int,
                    };
                    Ok((res, out_dt))
                }
            }
            ExpressionKind::Assignment { name, value } => {
                let (val, dt) =
                    Self::compile_expression(module, builder, strings, functions, value, vars)?;
                if let Some((var, _)) = vars.get(name) {
                    builder.def_var(*var, val);
                    Ok((val, dt))
                } else {
                    Err(CompileError::UndefinedVariable(name.clone()))
                }
            }
            ExpressionKind::Call { callee, arguments } => {
                if let ExpressionKind::Variable(ref name) = &*callee.kind {
                    if name == "println" && arguments.len() == 1 {
                        let (arg_val, arg_dt) = Self::compile_expression(
                            module,
                            builder,
                            strings,
                            functions,
                            &arguments[0],
                            vars,
                        )?;
                        if arg_dt == DotlinType::String {
                            let (func_id, _) = functions.get("println_str").unwrap();
                            let func_ref = module.declare_func_in_func(*func_id, &mut builder.func);
                            let call = builder.ins().call(func_ref, &[arg_val]);
                            let _ = builder.inst_results(call);
                        } else if arg_dt == DotlinType::Float {
                            let (func_id, _) = functions.get("println_f64").unwrap();
                            let func_ref = module.declare_func_in_func(*func_id, &mut builder.func);
                            let call = builder.ins().call(func_ref, &[arg_val]);
                            let _ = builder.inst_results(call);
                        } else {
                            let (func_id, _) = functions.get("println_i64").unwrap();
                            let func_ref = module.declare_func_in_func(*func_id, &mut builder.func);
                            let call = builder.ins().call(func_ref, &[arg_val]);
                            let _ = builder.inst_results(call);
                        }
                        Ok((builder.ins().iconst(types::I64, 0), DotlinType::Int))
                    } else if let Some((func_id, ret_type)) = functions.get(name) {
                        let mut args = Vec::new();
                        for arg in arguments {
                            let (v, _) = Self::compile_expression(
                                module, builder, strings, functions, arg, vars,
                            )?;
                            args.push(v);
                        }
                        let func_ref = module.declare_func_in_func(*func_id, &mut builder.func);
                        let call = builder.ins().call(func_ref, &args);
                        let results = builder.inst_results(call);
                        if results.is_empty() {
                            Ok((builder.ins().iconst(types::I64, 0), DotlinType::Int))
                        } else {
                            Ok((results[0], ret_type.unwrap_or(DotlinType::Int)))
                        }
                    } else {
                        Ok((builder.ins().iconst(types::I64, 0), DotlinType::Int))
                    }
                } else {
                    Ok((builder.ins().iconst(types::I64, 0), DotlinType::Int))
                }
            }
            ExpressionKind::Unary { operator, operand } => {
                let (val, dt) =
                    Self::compile_expression(module, builder, strings, functions, operand, vars)?;
                match operator {
                    UnaryOp::Minus => {
                        if dt == DotlinType::Float {
                            Ok((builder.ins().fneg(val), DotlinType::Float))
                        } else {
                            Ok((builder.ins().ineg(val), DotlinType::Int))
                        }
                    }
                    UnaryOp::Not => {
                        let zero = builder.ins().iconst(types::I8, 0);
                        Ok((
                            builder.ins().icmp(IntCC::Equal, val, zero),
                            DotlinType::Boolean,
                        ))
                    }
                }
            }
            ExpressionKind::MemberAccess { object, member } => {
                let (obj_val, obj_dt) =
                    Self::compile_expression(module, builder, strings, functions, object, vars)?;
                if obj_dt == DotlinType::String && member == "length" {
                    // String is pointer to [len: u64, data: ...u8]
                    Ok((
                        builder
                            .ins()
                            .load(types::I64, MemFlags::trusted(), obj_val, 0),
                        DotlinType::Int,
                    ))
                } else {
                    unreachable!(
                        "Type checker should have caught this: {:?} . {}",
                        obj_dt, member
                    )
                }
            }
            ExpressionKind::ArrayLiteral { elements } => {
                if elements.is_empty() {
                    // Create an empty array with capacity 10 as default
                    let (func_id, _) = functions.get("dotlin_array_new").unwrap();
                    let func_ref = module.declare_func_in_func(*func_id, &mut builder.func);
                    let capacity = builder.ins().iconst(types::I64, 10);
                    let element_size = builder.ins().iconst(types::I64, 8); // assuming 8-byte elements
                    let call = builder.ins().call(func_ref, &[element_size, capacity]);
                    let results = builder.inst_results(call);
                    Ok((results[0], DotlinType::Array))
                } else {
                    // Create array with capacity for all elements
                    let capacity = builder.ins().iconst(types::I64, elements.len() as i64);
                    let element_size = builder.ins().iconst(types::I64, 8); // assuming 8-byte elements
                    let func_id = functions.get("dotlin_array_new").unwrap().0;
                    let func_ref = module.declare_func_in_func(func_id, &mut builder.func);
                    let call = builder.ins().call(func_ref, &[element_size, capacity]);
                    let array_ptr = builder.inst_results(call)[0];
                    
                    // Add each element to the array
                    for (i, element) in elements.iter().enumerate() {
                        let (element_val, _) = Self::compile_expression(
                            module, builder, strings, functions, element, vars,
                        )?;
                        let index = builder.ins().iconst(types::I64, i as i64);
                        let set_func_id = functions.get("dotlin_array_set").unwrap().0;
                        let set_func_ref = module.declare_func_in_func(set_func_id, &mut builder.func);
                        let call = builder.ins().call(set_func_ref, &[array_ptr, index, element_val]);
                        let _ = builder.inst_results(call);
                    }
                    
                    Ok((array_ptr, DotlinType::Array))
                }
            }
            ExpressionKind::Index { array, index } => {
                let (array_ptr, _) = Self::compile_expression(
                    module, builder, strings, functions, array, vars,
                )?;
                let (index_val, _) = Self::compile_expression(
                    module, builder, strings, functions, index, vars,
                )?;
                
                // We need to determine if this is array or map indexing based on the type
                // For now, we'll default to array indexing, but we'll need to handle both
                let func_name = if array.resolved_type.as_ref().map_or(false, |t| {
                    matches!(t, Type::Map(_, _))
                }) {
                    "dotlin_map_get"
                } else {
                    "dotlin_array_get"
                };
                
                let func_id = functions.get(func_name).unwrap().0;
                let func_ref = module.declare_func_in_func(func_id, &mut builder.func);
                let call = builder.ins().call(func_ref, &[array_ptr, index_val]);
                let results = builder.inst_results(call);
                
                // Return type depends on the operation - for now default to Int
                let return_type = if func_name == "dotlin_map_get" {
                    DotlinType::Int  // Maps return values, for now using Int as placeholder
                } else {
                    DotlinType::Int  // Arrays return element values
                };
                
                Ok((results[0], return_type))
            }
            ExpressionKind::HashMapLiteral { pairs } => {
                // Create a new HashMap
                let func_id = functions.get("dotlin_map_new").unwrap().0;
                let func_ref = module.declare_func_in_func(func_id, &mut builder.func);
                let call = builder.ins().call(func_ref, &[]);
                let map_ptr = builder.inst_results(call)[0];
                
                // Add each key-value pair to the map
                for (key, value) in pairs {
                    let (key_val, _) = Self::compile_expression(
                        module, builder, strings, functions, key, vars,
                    )?;
                    let (value_val, _) = Self::compile_expression(
                        module, builder, strings, functions, value, vars,
                    )?;
                    
                    // Call dotlin_map_set to add the key-value pair
                    let set_func_id = functions.get("dotlin_map_set").unwrap().0;
                    let set_func_ref = module.declare_func_in_func(set_func_id, &mut builder.func);
                    let call = builder.ins().call(set_func_ref, &[map_ptr, key_val, value_val]);
                    let _ = builder.inst_results(call);
                }
                
                Ok((map_ptr, DotlinType::Int)) // HashMap is represented as a pointer (Int)
            }
        }
    }
}
