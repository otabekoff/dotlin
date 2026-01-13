use std::collections::HashMap;
use std::io::{self, Write};

use dotlin_parser::ast::{Expr, Pattern, Stmt};

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Number(f64),
    Bool(bool),
    Array(Vec<Value>),
    Function(Vec<String>, Box<Expr>, Env),
    Unit,
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Str(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Array(a) => format!(
                "[{}]",
                a.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Function(_, _, _) => "<function>".into(),
            Value::Unit => "".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Env {
    // name -> (maybe-initialized-value, is_mutable)
    vars: HashMap<String, (Option<Value>, bool)>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn has(&self, k: &str) -> bool {
        self.vars.contains_key(k)
    }

    pub fn get(&self, k: &str) -> Option<&Value> {
        self.vars.get(k).and_then(|(v, _)| v.as_ref())
    }

    pub fn declare(&mut self, k: String, v: Option<Value>, is_mut: bool) {
        self.vars.insert(k, (v, is_mut));
    }

    pub fn assign(&mut self, k: &str, val: Value) -> Result<(), String> {
        if let Some(entry) = self.vars.get_mut(k) {
            let is_mut = entry.1;
            if is_mut {
                entry.0 = Some(val);
                Ok(())
            } else {
                if entry.0.is_none() {
                    entry.0 = Some(val);
                    Ok(())
                } else {
                    Err(format!("cannot assign to immutable variable {}", k))
                }
            }
        } else {
            Err(format!("unknown identifier {}", k))
        }
    }
}

pub struct Interpreter {
    pub funcs: HashMap<String, (Vec<String>, Vec<Stmt>, Option<Expr>, Option<String>)>,
    pub env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            env: Env::new(),
        }
    }

    pub fn register_fn(
        &mut self,
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        expr_body: Option<Expr>,
        return_type: Option<String>,
    ) {
        self.funcs
            .insert(name, (params, body, expr_body, return_type));
    }

    fn value_matches_type(&self, v: &Value, tn: &str) -> bool {
        match tn {
            "Long" | "Int" | "Number" => matches!(v, Value::Number(_)),
            "String" | "Any" => true,
            "Boolean" | "Bool" => matches!(v, Value::Bool(_)),
            "Unit" => matches!(v, Value::Unit),
            _ => false,
        }
    }

    fn is_truthy(&self, v: &Value) -> bool {
        match v {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Function(_, _, _) => true,
            Value::Unit => false,
        }
    }

    fn matches_pattern(
        &mut self,
        pat: Pattern,
        value: Option<Value>,
    ) -> Result<Option<Vec<(String, Value)>>, String> {
        match pat {
            Pattern::Else(_) => Ok(Some(vec![])),
            Pattern::LitNumber(n, _) => {
                if let Some(Value::Number(vn)) = value {
                    Ok(if vn.to_string() == n {
                        Some(vec![])
                    } else {
                        None
                    })
                } else {
                    Ok(None)
                }
            }
            Pattern::LitStr(s, _) => {
                if let Some(Value::Str(vs)) = value {
                    Ok(if vs == s { Some(vec![]) } else { None })
                } else {
                    Ok(None)
                }
            }
            Pattern::LitBool(b, _) => {
                if let Some(Value::Bool(vb)) = value {
                    Ok(if vb == b { Some(vec![]) } else { None })
                } else {
                    Ok(None)
                }
            }
            Pattern::IsType(tn, _) => {
                if let Some(v) = value {
                    let ok = match tn.as_str() {
                        "Long" | "Int" | "Number" => matches!(v, Value::Number(_)),
                        "String" | "Any" => true,
                        "Boolean" | "Bool" => matches!(v, Value::Bool(_)),
                        _ => false,
                    };
                    Ok(if ok { Some(vec![]) } else { None })
                } else {
                    Ok(None)
                }
            }
            Pattern::IsBind(type_name, bind_name, _) => {
                if let Some(v) = value {
                    let ok = match type_name.as_str() {
                        "Long" | "Int" | "Number" => matches!(v, Value::Number(_)),
                        "String" | "Any" => true,
                        "Boolean" | "Bool" => matches!(v, Value::Bool(_)),
                        _ => false,
                    };
                    if ok {
                        return Ok(Some(vec![(bind_name, v)]));
                    }
                    Ok(None)
                } else {
                    Ok(None)
                }
            }
            Pattern::NotIsType(tn, _) => {
                if let Some(v) = value {
                    let ok = match tn.as_str() {
                        "String" => !matches!(v, Value::Str(_)),
                        _ => false,
                    };
                    Ok(if ok { Some(vec![]) } else { None })
                } else {
                    Ok(None)
                }
            }
            Pattern::InExpr(expr, _) => {
                if let Some(v) = value {
                    let rhs = self.eval_expr(expr)?;
                    match rhs {
                        Value::Array(arr) => {
                            Ok(if arr.iter().any(|it| it.to_string() == v.to_string()) {
                                Some(vec![])
                            } else {
                                None
                            })
                        }
                        _ => Ok(None),
                    }
                } else {
                    Ok(None)
                }
            }
            Pattern::Range(sstart, send, _) => {
                if let Some(Value::Number(v)) = value {
                    let ia = sstart.parse::<i64>().unwrap_or(v as i64);
                    let ib = send.parse::<i64>().unwrap_or(v as i64);
                    Ok(if (v as i64) >= ia && (v as i64) <= ib {
                        Some(vec![])
                    } else {
                        None
                    })
                } else {
                    Ok(None)
                }
            }
            Pattern::Array(pats, _) => {
                if let Some(Value::Array(arr)) = value {
                    if pats.len() != arr.len() {
                        return Ok(None);
                    }
                    let mut bindings: Vec<(String, Value)> = Vec::new();
                    for (i, p) in pats.into_iter().enumerate() {
                        match self.matches_pattern(p, Some(arr[i].clone()))? {
                            Some(mut b) => bindings.append(&mut b),
                            None => return Ok(None),
                        }
                    }
                    Ok(Some(bindings))
                } else {
                    Ok(None)
                }
            }
            Pattern::Bind(name, _) => {
                if let Some(v) = value {
                    Ok(Some(vec![(name, v)]))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub fn eval_expr(&mut self, e: Expr) -> Result<Value, String> {
        match e {
            Expr::LitStr(s, _) => Ok(Value::Str(s)),
            Expr::LitNumber(n, _) => {
                let parsed = n
                    .parse::<f64>()
                    .map_err(|_| format!("invalid number {}", n))?;
                Ok(Value::Number(parsed))
            }
            Expr::LitBool(b, _) => Ok(Value::Bool(b)),

            Expr::Lambda { params, body, .. } => {
                Ok(Value::Function(params, body, self.env.clone()))
            }

            Expr::Ident(name, _) => {
                if let Some(v) = self.env.get(&name) {
                    Ok(v.clone())
                } else if self.env.has(&name) {
                    Err(format!("variable {} is not initialized", name))
                } else {
                    Err(format!("unknown identifier {}", name))
                }
            }

            Expr::Member { receiver, name, .. } => {
                let recv = self.eval_expr(*receiver)?;
                match name.as_str() {
                    "size" | "length" => match recv {
                        Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
                        Value::Str(s) => Ok(Value::Number(s.len() as f64)),
                        _ => Err("length/size not supported on this receiver".into()),
                    },
                    "indices" => match recv {
                        Value::Array(arr) => {
                            let mut v = Vec::new();
                            for i in 0..arr.len() {
                                v.push(Value::Number(i as f64));
                            }
                            Ok(Value::Array(v))
                        }
                        Value::Str(s) => {
                            let mut v = Vec::new();
                            for i in 0..s.len() {
                                v.push(Value::Number(i as f64));
                            }
                            Ok(Value::Array(v))
                        }
                        _ => Err("indices not supported on this receiver".into()),
                    },
                    "lastIndex" => match recv {
                        Value::Array(arr) => {
                            if arr.is_empty() {
                                Ok(Value::Number(-1.0))
                            } else {
                                Ok(Value::Number((arr.len() - 1) as f64))
                            }
                        }
                        Value::Str(s) => {
                            if s.is_empty() {
                                Ok(Value::Number(-1.0))
                            } else {
                                Ok(Value::Number((s.len() - 1) as f64))
                            }
                        }
                        _ => Err("lastIndex not supported on this receiver".into()),
                    },
                    _ => Err(format!("unknown member {}", name)),
                }
            }

            Expr::Call { callee, args, .. } => match *callee {
                Expr::Ident(name, _) => self.call_ident(&name, args),
                Expr::Member { receiver, name, .. } => self.call_member(*receiver, &name, args),
                Expr::Lambda { .. } => {
                    // calling a lambda directly: evaluate callee to function value and invoke
                    let fval = self.eval_expr(*callee)?;
                    match fval {
                        Value::Function(params, body, fenv) => {
                            // evaluate args
                            let mut evaluated = Vec::new();
                            for a in args {
                                evaluated.push(self.eval_expr(a)?);
                            }
                            // call closure
                            let _original_env = self.env.clone();
                            let mut fn_env = fenv.clone();
                            for (i, pname) in params.iter().enumerate() {
                                let val = evaluated.get(i).cloned().unwrap_or(Value::Unit);
                                fn_env.declare(pname.clone(), Some(val), false);
                            }
                            let previous_env = std::mem::replace(&mut self.env, fn_env);
                            let res = self.eval_expr(*body);
                            let _fn_env_after = std::mem::replace(&mut self.env, previous_env);
                            res
                        }
                        _ => Err("call target not a function".into()),
                    }
                }
                _ => Err("call target not supported".into()),
            },

            Expr::Index { target, index, .. } => {
                let tv = self.eval_expr(*target)?;
                match tv {
                    Value::Array(arr) => {
                        if index < arr.len() {
                            Ok(arr[index].clone())
                        } else {
                            Err("index out of bounds".into())
                        }
                    }
                    _ => Err("attempt to index non-array value".into()),
                }
            }

            Expr::Unary { op, expr, .. } => {
                let v = self.eval_expr(*expr)?;
                match op.as_str() {
                    "!" => Ok(Value::Bool(!self.is_truthy(&v))),
                    other => Err(format!("unknown unary operator {}", other)),
                }
            }

            Expr::Binary {
                left, op, right, ..
            } => match op.as_str() {
                "=" | "+=" => {
                    let rval = self.eval_expr(*right)?;
                    match *left {
                        Expr::Ident(name, _) => {
                            if op == "=" {
                                self.env.assign(&name, rval.clone())?;
                                Ok(rval)
                            } else {
                                if let Some(cur) = self.env.get(&name) {
                                    let newv = match (cur.clone(), rval.clone()) {
                                        (Value::Number(a), Value::Number(b)) => {
                                            Value::Number(a + b)
                                        }
                                        (Value::Str(a), Value::Str(b)) => Value::Str(a + &b),
                                        (Value::Str(a), v) => Value::Str(a + &v.to_string()),
                                        (v, Value::Str(b)) => Value::Str(v.to_string() + &b),
                                        _ => return Err("unsupported += operand types".into()),
                                    };
                                    self.env.assign(&name, newv.clone())?;
                                    Ok(newv)
                                } else {
                                    Err(format!("unknown identifier {}", name))
                                }
                            }
                        }
                        Expr::Index { target, index, .. } => {
                            if let Expr::Ident(varname, _) = &*target {
                                let varname = varname.clone();
                                let t = self.eval_expr(*target)?;
                                match t {
                                    Value::Array(mut arr) => {
                                        if index >= arr.len() {
                                            return Err("index out of bounds".into());
                                        }
                                        if op == "=" {
                                            arr[index] = rval.clone();
                                            self.env.assign(&varname, Value::Array(arr))?;
                                            Ok(rval)
                                        } else {
                                            Err("compound assignment on indexed lvalue not supported".into())
                                        }
                                    }
                                    _ => Err("attempt to index non-array value".into()),
                                }
                            } else {
                                Err("assignment to complex lvalue not supported".into())
                            }
                        }
                        _ => Err("left-hand side of assignment must be identifier or index".into()),
                    }
                }
                _ => {
                    let l = self.eval_expr(*left)?;
                    let r = self.eval_expr(*right)?;
                    match op.as_str() {
                        ".." => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => {
                                let mut v = Vec::new();
                                let ia = a as i64;
                                let ib = b as i64;
                                if ia <= ib {
                                    for x in ia..=ib {
                                        v.push(Value::Number(x as f64));
                                    }
                                } else {
                                    for x in (ib..=ia).rev() {
                                        v.push(Value::Number(x as f64));
                                    }
                                }
                                Ok(Value::Array(v))
                            }
                            _ => Err("range operator requires integer numeric operands".into()),
                        },
                        "downTo" => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => {
                                let mut v = Vec::new();
                                let ia = a as i64;
                                let ib = b as i64;
                                if ia >= ib {
                                    for x in (ib..=ia).rev() {
                                        v.push(Value::Number(x as f64));
                                    }
                                } else {
                                    for x in ia..=ib {
                                        v.push(Value::Number(x as f64));
                                    }
                                }
                                Ok(Value::Array(v))
                            }
                            _ => Err("downTo requires integer numeric operands".into()),
                        },
                        "until" => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => {
                                let mut v = Vec::new();
                                let ia = a as i64;
                                let ib = b as i64;
                                if ia < ib {
                                    for x in ia..ib {
                                        v.push(Value::Number(x as f64));
                                    }
                                } else if ia > ib {
                                    for x in (ib..ia).rev() {
                                        v.push(Value::Number(x as f64));
                                    }
                                }
                                Ok(Value::Array(v))
                            }
                            _ => Err("until requires integer numeric operands".into()),
                        },
                        "in" => match r {
                            Value::Array(arr) => Ok(Value::Bool(
                                arr.iter().any(|it| it.to_string() == l.to_string()),
                            )),
                            _ => Err("in requires an array on the right side".into()),
                        },
                        "step" => {
                            // apply step to lhs which should be an Array
                            match (l, r) {
                                (Value::Array(arr), Value::Number(stepf)) => {
                                    let step_i = stepf as i64;
                                    if step_i == 0 {
                                        return Err("step must be non-zero".into());
                                    }
                                    let mut v = Vec::new();
                                    if step_i > 0 {
                                        let step = step_i as usize;
                                        let mut i = 0usize;
                                        while i < arr.len() {
                                            v.push(arr[i].clone());
                                            i = i.saturating_add(step);
                                        }
                                    } else {
                                        // negative step: iterate from end backwards
                                        let step = (-step_i) as usize;
                                        if arr.is_empty() {
                                            return Ok(Value::Array(v));
                                        }
                                        let mut idx = (arr.len() - 1) as i64;
                                        while idx >= 0 {
                                            v.push(arr[idx as usize].clone());
                                            idx -= step as i64;
                                        }
                                    }
                                    Ok(Value::Array(v))
                                }
                                _ => {
                                    Err("step requires an array on the left and numeric step"
                                        .into())
                                }
                            }
                        }
                        "+" => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                            (lv, rv) => Ok(Value::Str(lv.to_string() + &rv.to_string())),
                        },
                        "-" => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                            _ => Err("- operator requires numbers".into()),
                        },
                        "*" => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                            _ => Err("* operator requires numbers".into()),
                        },
                        "/" => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
                            _ => Err("/ operator requires numbers".into()),
                        },
                        "==" => Ok(Value::Bool(l.to_string() == r.to_string())),
                        "!=" => Ok(Value::Bool(l.to_string() != r.to_string())),
                        "<" => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a < b)),
                            _ => Err("< operator requires comparable types".into()),
                        },
                        ">" => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a > b)),
                            _ => Err("> operator requires comparable types".into()),
                        },
                        "<=" => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a <= b)),
                            _ => Err("<= operator requires comparable types".into()),
                        },
                        ">=" => match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
                            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a >= b)),
                            _ => Err(">= operator requires comparable types".into()),
                        },
                        "&&" => {
                            if !self.is_truthy(&l) {
                                return Ok(Value::Bool(false));
                            }
                            Ok(Value::Bool(self.is_truthy(&r)))
                        }
                        "||" => {
                            if self.is_truthy(&l) {
                                return Ok(Value::Bool(true));
                            }
                            Ok(Value::Bool(self.is_truthy(&r)))
                        }
                        other => Err(format!("unknown operator {}", other)),
                    }
                }
            },

            Expr::When {
                scrutinee, arms, ..
            } => {
                let value = if let Some(s) = scrutinee {
                    Some(self.eval_expr(*s)?)
                } else {
                    None
                };
                for (pat, arm_expr) in arms {
                    match self.matches_pattern(pat, value.clone())? {
                        Some(bindings) => {
                            // apply bindings temporarily
                            let mut saved: Vec<(String, Option<(Option<Value>, bool)>)> =
                                Vec::new();
                            for (k, v) in bindings.iter() {
                                if let Some(prev) = self.env.vars.remove(k) {
                                    saved.push((k.clone(), Some(prev)));
                                } else {
                                    saved.push((k.clone(), None));
                                }
                                self.env.declare(k.clone(), Some(v.clone()), false);
                            }
                            // evaluate arm
                            let res = self.eval_expr(arm_expr);
                            // restore saved bindings
                            for (k, maybe) in saved.into_iter() {
                                if let Some(entry) = maybe {
                                    self.env.vars.insert(k, entry);
                                } else {
                                    self.env.vars.remove(&k);
                                }
                            }
                            return res;
                        }
                        None => continue,
                    }
                }
                Ok(Value::Unit)
            }
        }
    }

    fn call_ident(&mut self, name: &str, args: Vec<Expr>) -> Result<Value, String> {
        match name {
            "println" => {
                let mut out = String::new();
                for (i, a) in args.into_iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    let v = self.eval_expr(a)?;
                    out.push_str(&v.to_string());
                }
                println!("{}", out);
                Ok(Value::Unit)
            }
            "print" => {
                let mut out = String::new();
                for (i, a) in args.into_iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    let v = self.eval_expr(a)?;
                    out.push_str(&v.to_string());
                }
                print!("{}", out);
                io::stdout().flush().map_err(|e| e.to_string())?;
                Ok(Value::Unit)
            }
            "readln" | "read" => {
                let mut s = String::new();
                io::stdin().read_line(&mut s).map_err(|e| e.to_string())?;
                if s.ends_with('\n') {
                    s.pop();
                    if s.ends_with('\r') {
                        s.pop();
                    }
                }
                Ok(Value::Str(s))
            }
            "contentToString" => {
                if args.len() != 1 {
                    return Err("contentToString expects 1 argument".into());
                }
                let v = self.eval_expr(args.into_iter().next().unwrap())?;
                match v {
                    Value::Array(arr) => Ok(Value::Str(format!(
                        "[{}]",
                        arr.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))),
                    _ => Err("contentToString expects an array".into()),
                }
            }
            "joinToString" => {
                if args.len() != 1 {
                    return Err("joinToString expects 1 argument".into());
                }
                let v = self.eval_expr(args.into_iter().next().unwrap())?;
                match v {
                    Value::Array(arr) => Ok(Value::Str(
                        arr.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                    )),
                    _ => Err("joinToString expects an array".into()),
                }
            }
            "toCString" | "toKotlinString" => {
                if args.len() != 1 {
                    return Err("expects 1 argument".into());
                }
                match self.eval_expr(args.into_iter().next().unwrap())? {
                    Value::Str(s) => Ok(Value::Str(s)),
                    _ => Err("expected string".into()),
                }
            }
            "listOf" => {
                let mut vals = Vec::new();
                for a in args {
                    vals.push(self.eval_expr(a)?);
                }
                Ok(Value::Array(vals))
            }
            "setOf" => {
                let mut vals = Vec::new();
                for a in args {
                    vals.push(self.eval_expr(a)?);
                }
                Ok(Value::Array(vals))
            }
            "mapOf" => {
                // Expect even number of args: key, value, key, value...
                let mut pairs: Vec<Value> = Vec::new();
                let mut evals: Vec<Value> = Vec::new();
                for a in args {
                    evals.push(self.eval_expr(a)?);
                }
                if evals.len() % 2 != 0 {
                    return Err("mapOf expects an even number of arguments".into());
                }
                let mut i = 0;
                while i < evals.len() {
                    let k = evals[i].clone();
                    let v = evals[i + 1].clone();
                    pairs.push(Value::Array(vec![k, v]));
                    i += 2;
                }
                Ok(Value::Array(pairs))
            }
            "filterStartsWith" => {
                // filterStartsWith(collection, prefix)
                if args.len() != 2 {
                    return Err("filterStartsWith expects 2 arguments".into());
                }
                let col = self.eval_expr(args[0].clone())?;
                let prefix = self.eval_expr(args[1].clone())?;
                let mut out = Vec::new();
                match (col, prefix) {
                    (Value::Array(arr), Value::Str(p)) => {
                        for it in arr.into_iter() {
                            if it.to_string().starts_with(&p) {
                                out.push(it);
                            }
                        }
                        Ok(Value::Array(out))
                    }
                    _ => Err("filterStartsWith expects (array, string)".into()),
                }
            }
            "mapToUpper" => {
                // mapToUpper(collection)
                if args.len() != 1 {
                    return Err("mapToUpper expects 1 argument".into());
                }
                let col = self.eval_expr(args[0].clone())?;
                match col {
                    Value::Array(arr) => {
                        let mut out = Vec::new();
                        for it in arr.into_iter() {
                            out.push(Value::Str(it.to_string().to_uppercase()));
                        }
                        Ok(Value::Array(out))
                    }
                    _ => Err("mapToUpper expects an array".into()),
                }
            }
            other => {
                if let Some((params, body, expr_body, return_type)) = self.funcs.get(other).cloned()
                {
                    let mut evaluated = Vec::new();
                    for a in args {
                        evaluated.push(self.eval_expr(a)?);
                    }
                    // if expression-bodied function, evaluate expression in function env
                    if let Some(expr) = expr_body {
                        // create function env and evaluate expr
                        let original_env = self.env.clone();
                        let mut fn_env = original_env.clone();
                        if params.len() == 1 && params[0] == "args" {
                            fn_env.declare(
                                "args".into(),
                                Some(Value::Array(evaluated.clone())),
                                false,
                            );
                        } else {
                            for (i, pname) in params.iter().enumerate() {
                                let val = evaluated.get(i).cloned().unwrap_or(Value::Unit);
                                fn_env.declare(pname.clone(), Some(val), false);
                            }
                        }
                        let previous_env = std::mem::replace(&mut self.env, fn_env);
                        let result = self.eval_expr(expr)?;
                        if let Some(rt) = return_type.clone() {
                            if !self.value_matches_type(&result, &rt) {
                                return Err(format!(
                                    "function {} returned value with unexpected type (expected {})",
                                    other, rt
                                ));
                            }
                        }
                        let fn_env_after = std::mem::replace(&mut self.env, previous_env);
                        // merge top-level changes
                        for (k, (v, is_mut)) in fn_env_after.vars.into_iter() {
                            if self.env.vars.contains_key(&k) {
                                self.env.vars.insert(k, (v, is_mut));
                            }
                        }
                        Ok(result)
                    } else {
                        let ret = self.run_function(&params, &evaluated, body)?;
                        if let Some(v) = ret {
                            if let Some(rt) = return_type.clone() {
                                if !self.value_matches_type(&v, &rt) {
                                    return Err(format!(
                                        "function {} returned value with unexpected type (expected {})",
                                        other, rt
                                    ));
                                }
                            }
                            Ok(v)
                        } else {
                            // void return -> Unit
                            if let Some(rt) = return_type.clone() {
                                if rt != "Unit" {
                                    return Err(format!(
                                        "function {} returned Unit but expected {}",
                                        other, rt
                                    ));
                                }
                            }
                            Ok(Value::Unit)
                        }
                    }
                } else {
                    Err(format!("unknown function {}", other))
                }
            }
        }
    }

    fn call_member(
        &mut self,
        receiver: Expr,
        name: &str,
        args: Vec<Expr>,
    ) -> Result<Value, String> {
        let recv = self.eval_expr(receiver)?;
        match name {
            "contentToString" => match recv {
                Value::Array(arr) => Ok(Value::Str(format!(
                    "[{}]",
                    arr.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))),
                _ => Err("contentToString receiver must be an array".into()),
            },
            "joinToString" => match recv {
                Value::Array(arr) => {
                    let sep = if args.len() == 1 {
                        match self.eval_expr(args.into_iter().next().unwrap())? {
                            Value::Str(s) => s,
                            _ => ", ".into(),
                        }
                    } else {
                        ", ".into()
                    };
                    Ok(Value::Str(
                        arr.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join(&sep),
                    ))
                }
                _ => Err("joinToString receiver must be an array".into()),
            },
            "toCString" | "toKotlinString" => match recv {
                Value::Str(s) => Ok(Value::Str(s)),
                _ => Err("toCString/toKotlinString expects a string receiver".into()),
            },
            "filter" => match recv {
                Value::Array(arr) => {
                    if args.len() != 1 {
                        return Err("filter expects 1 argument (predicate)".into());
                    }
                    let pred = self.eval_expr(args[0].clone())?;
                    let mut out = Vec::new();
                    for it in arr.into_iter() {
                        let res = self.call_closure(pred.clone(), it.clone())?;
                        if let Value::Bool(b) = res {
                            if b {
                                out.push(it);
                            }
                        }
                    }
                    Ok(Value::Array(out))
                }
                _ => Err("filter receiver must be an array".into()),
            },
            "map" => match recv {
                Value::Array(arr) => {
                    if args.len() != 1 {
                        return Err("map expects 1 argument (transform)".into());
                    }
                    let mapper = self.eval_expr(args[0].clone())?;
                    let mut out = Vec::new();
                    for it in arr.into_iter() {
                        let res = self.call_closure(mapper.clone(), it)?;
                        out.push(res);
                    }
                    Ok(Value::Array(out))
                }
                _ => Err("map receiver must be an array".into()),
            },
            other => Err(format!("unknown method {}", other)),
        }
    }

    // helper to invoke a closure Value::Function with a single argument
    fn call_closure(&mut self, fv: Value, arg: Value) -> Result<Value, String> {
        match fv {
            Value::Function(params, body, fenv) => {
                let _original_env = self.env.clone();
                let mut fn_env = fenv.clone();
                if params.len() >= 1 {
                    fn_env.declare(params[0].clone(), Some(arg), false);
                }
                let previous_env = std::mem::replace(&mut self.env, fn_env);
                let res = self.eval_expr(*body);
                let _fn_env_after = std::mem::replace(&mut self.env, previous_env);
                res
            }
            _ => Err("value is not a function".into()),
        }
    }

    pub fn run_function(
        &mut self,
        params: &Vec<String>,
        args: &Vec<Value>,
        body: Vec<Stmt>,
    ) -> Result<Option<Value>, String> {
        // create function environment that inherits top-level variables
        let original_env = self.env.clone();
        let mut fn_env = original_env.clone();
        if params.len() == 1 && params[0] == "args" {
            fn_env.declare("args".into(), Some(Value::Array(args.clone())), false);
        } else {
            for (i, pname) in params.iter().enumerate() {
                let val = args.get(i).cloned().unwrap_or(Value::Unit);
                fn_env.declare(pname.clone(), Some(val), false);
            }
        }

        // replace current env with function env and run
        let previous_env = std::mem::replace(&mut self.env, fn_env);
        let result = self.run_block(body)?;

        // take the mutated function env back and restore previous env
        let fn_env_after = std::mem::replace(&mut self.env, previous_env);

        // merge changes back into restored (top-level) env for variables that exist there
        for (k, (v, is_mut)) in fn_env_after.vars.into_iter() {
            if self.env.vars.contains_key(&k) {
                self.env.vars.insert(k, (v, is_mut));
            }
        }

        Ok(result)
    }

    pub fn run_block(&mut self, stmts: Vec<Stmt>) -> Result<Option<Value>, String> {
        for s in stmts {
            match s {
                Stmt::ExprStmt(e, _) => {
                    let _ = self.eval_expr(e)?;
                }
                Stmt::VarDecl {
                    is_mut,
                    name,
                    type_name,
                    init,
                    ..
                } => {
                    if init.is_none() && type_name.is_none() {
                        return Err(format!(
                            "variable '{}' declared without initializer must include an explicit type",
                            name
                        ));
                    }
                    let v_opt = if let Some(e) = init {
                        Some(self.eval_expr(e)?)
                    } else {
                        None
                    };
                    self.env.declare(name, v_opt, is_mut);
                }
                Stmt::Block(bs, _) => {
                    if let Some(rv) = self.run_block(bs)? {
                        return Ok(Some(rv));
                    }
                }
                Stmt::For {
                    var,
                    iterable,
                    body,
                    ..
                } => {
                    let iter_v = self.eval_expr(iterable)?;
                    match iter_v {
                        Value::Array(arr) => {
                            let prev = self.env.vars.remove(&var);
                            for item in arr.into_iter() {
                                self.env.declare(var.clone(), Some(item.clone()), false);
                                if let Some(rv) = self.run_block(body.clone())? {
                                    if let Some(entry) = prev.clone() {
                                        self.env.vars.insert(var.clone(), entry);
                                    } else {
                                        self.env.vars.remove(&var);
                                    }
                                    return Ok(Some(rv));
                                }
                            }
                            if let Some(entry) = prev {
                                self.env.vars.insert(var, entry);
                            } else {
                                self.env.vars.remove(&var);
                            }
                        }
                        _ => return Err("for loop iterable must be array".into()),
                    }
                }
                Stmt::While { cond, body, .. } => loop {
                    let cv = self.eval_expr(cond.clone())?;
                    match cv {
                        Value::Bool(b) => {
                            if !b {
                                break;
                            }
                        }
                        _ => return Err("while condition must be boolean".into()),
                    }
                    if let Some(rv) = self.run_block(body.clone())? {
                        return Ok(Some(rv));
                    }
                },
                Stmt::Return(expr_opt, _) => {
                    let ret = if let Some(e) = expr_opt {
                        self.eval_expr(e)?
                    } else {
                        Value::Unit
                    };
                    return Ok(Some(ret));
                }
            }
        }
        Ok(None)
    }
}
