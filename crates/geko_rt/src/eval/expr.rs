/// Imports
use crate::{
    errors::RuntimeError,
    interpreter::Interpreter,
    refs::{MutRef, RealmRef, Ref},
    rt::{
        flow::{ControlFlow, Flow},
        realm::Realm,
        value::{Bound, Callable, Class, Closure, Function, Instance, Method, Native, Value},
    },
};
use geko_common::{bail, bug};
use geko_ir::{
    atom::{BinOp, Lit, UnaryOp},
    expr::Expression,
    stmt::Block,
};
use geko_lex::token::Span;
use std::{cell::RefCell, collections::HashMap};

/// Expression evaluation implementation
impl<'io> Interpreter<'io> {
    /// Evaluates literal expression
    pub(crate) fn eval_lit(&self, lit: &Lit) -> Flow<Value> {
        // Matching literal
        Ok(match lit {
            Lit::Number(number) => {
                if number.contains('.') {
                    Value::Float(number.parse::<f64>().unwrap())
                } else {
                    Value::Int(number.parse::<i64>().unwrap())
                }
            }
            Lit::String(string) => Value::String(string.clone()),
            Lit::Bool(bool) => Value::Bool(bool.parse::<bool>().unwrap()),
            Lit::Null => Value::Null,
        })
    }

    /// Performs binary operation over floats
    fn float_bin_op(span: &Span, a: f64, b: f64, op: BinOp) -> Value {
        // Matching operator
        match op {
            BinOp::Gt => Value::Bool(a > b),
            BinOp::Ge => Value::Bool(a >= b),
            BinOp::Lt => Value::Bool(a < b),
            BinOp::Le => Value::Bool(a <= b),
            BinOp::Eq => Value::Bool(a == b),
            BinOp::Ne => Value::Bool(a != b),
            BinOp::Add => Value::Float(a + b),
            BinOp::Sub => Value::Float(a - b),
            BinOp::Mul => Value::Float(a * b),
            BinOp::Div => {
                if b > 0.0 {
                    Value::Float(a / b)
                } else {
                    bail!(RuntimeError::ZeroDivision {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }
            }
            BinOp::Mod => Value::Float(a % b),
            _ => bail!(RuntimeError::InvalidBinOp {
                op,
                a: Value::Float(a),
                b: Value::Float(b),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }
    }

    /// Performs binary operation over ints
    fn int_bin_op(span: &Span, a: i64, b: i64, op: BinOp) -> Value {
        // Matching operator
        match op {
            BinOp::Gt => Value::Bool(a > b),
            BinOp::Ge => Value::Bool(a >= b),
            BinOp::Lt => Value::Bool(a < b),
            BinOp::Le => Value::Bool(a <= b),
            BinOp::Eq => Value::Bool(a == b),
            BinOp::Ne => Value::Bool(a != b),
            BinOp::Add => Value::Int(a + b),
            BinOp::Sub => Value::Int(a - b),
            BinOp::Mul => Value::Int(a * b),
            BinOp::Div => {
                if b > 0 {
                    Value::Int(a / b)
                } else {
                    bail!(RuntimeError::ZeroDivision {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }
            }
            BinOp::Mod => Value::Int(a % b),
            _ => bail!(RuntimeError::InvalidBinOp {
                op,
                a: Value::Int(a),
                b: Value::Int(b),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }
    }

    /// Performs binary operation over bools
    fn bool_bin_op(span: &Span, a: bool, b: bool, op: BinOp) -> Value {
        // Matching operator
        match op {
            BinOp::And => Value::Bool(a && b),
            BinOp::Or => Value::Bool(a || b),
            BinOp::Gt => Value::Bool(a & !b),
            BinOp::Ge => Value::Bool(a >= b),
            BinOp::Lt => Value::Bool(!a & b),
            BinOp::Le => Value::Bool(a <= b),
            BinOp::Eq => Value::Bool(a == b),
            BinOp::Ne => Value::Bool(a != b),
            BinOp::BitAnd => Value::Bool(a & b),
            BinOp::BitOr => Value::Bool(a | b),
            BinOp::Xor => Value::Bool(a ^ b),
            _ => bail!(RuntimeError::InvalidBinOp {
                op,
                a: Value::Bool(a),
                b: Value::Bool(b),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }
    }

    /// Performs binary operation over strings
    fn string_bin_op(span: &Span, a: String, b: String, op: BinOp) -> Value {
        // Matching operator
        match op {
            BinOp::Ge => Value::Bool(a >= b),
            BinOp::Le => Value::Bool(a <= b),
            BinOp::Eq => Value::Bool(a == b),
            BinOp::Ne => Value::Bool(a != b),
            BinOp::Add => Value::String(format!("{a}{b}")),
            _ => bail!(RuntimeError::InvalidBinOp {
                op,
                a: Value::String(a),
                b: Value::String(b),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }
    }

    /// Performs binary operation over values
    pub(crate) fn perform_bin_op(
        &self,
        span: &Span,
        left: Value,
        right: Value,
        op: BinOp,
    ) -> Value {
        // Invalid binary operation error
        let invalid_bin_op = |a, b| {
            bail!(RuntimeError::InvalidBinOp {
                op,
                a,
                b,
                src: span.0.clone(),
                span: span.1.clone().into()
            })
        };

        // Matching binary operator
        match (left, right) {
            // Impls and non-impls on any values
            (a, b) if matches!(op, BinOp::Impls) => Value::Bool(Self::is_impls(span, a, b)),
            (a, b) if matches!(op, BinOp::NotImpls) => Value::Bool(!Self::is_impls(span, a, b)),

            // Binary operation over numbers
            (Value::Int(a), Value::Int(b)) => Self::int_bin_op(span, a, b, op),
            (Value::Int(a), Value::Float(b)) => Self::float_bin_op(span, a as f64, b, op),
            (Value::Float(a), Value::Int(b)) => Self::float_bin_op(span, a, b as f64, op),
            (Value::Float(a), Value::Float(b)) => Self::float_bin_op(span, a, b, op),

            // Binary operation over bools
            (Value::Bool(a), Value::Bool(b)) => Self::bool_bin_op(span, a, b, op),

            // Binary operation over strings
            (Value::String(a), Value::String(b)) => Self::string_bin_op(span, a, b, op),

            // Binary operation over any other values
            (a, b) => match op {
                BinOp::Eq => Value::Bool(a == b),
                BinOp::Ne => Value::Bool(a != b),
                _ => invalid_bin_op(a, b),
            },
        }
    }

    /// Evaluates binary expression
    pub(crate) fn eval_bin(
        &mut self,
        span: &Span,
        op: BinOp,
        left: &Expression,
        right: &Expression,
    ) -> Flow<Value> {
        // Evaluating lhs and rhs
        let left = self.eval(left)?;
        let right = self.eval(right)?;

        // Performing bin op
        Ok(self.perform_bin_op(span, left, right, op))
    }

    /// Performs unary operation over values
    pub(crate) fn perform_unary_op(span: &Span, value: Value, op: UnaryOp) -> Value {
        // Invalid unary op
        let invalid_unary_op = || {
            bail!(RuntimeError::InvalidUnaryOp {
                op,
                value: value.clone(),
                src: span.0.clone(),
                span: span.1.clone().into()
            });
        };

        // Matching left and right types
        match value {
            Value::Bool(a) => match op {
                UnaryOp::Bang => Value::Bool(!a),
                _ => invalid_unary_op(),
            },
            Value::Int(a) => match op {
                UnaryOp::Neg => Value::Int(-a),
                _ => invalid_unary_op(),
            },
            Value::Float(a) => match op {
                UnaryOp::Neg => Value::Float(-a),
                _ => invalid_unary_op(),
            },
            _ => invalid_unary_op(),
        }
    }

    /// Evaluates unary expression
    pub(crate) fn eval_unary(
        &mut self,
        span: &Span,
        op: UnaryOp,
        value: &Expression,
    ) -> Flow<Value> {
        // Evaluating value
        let value = self.eval(value)?;

        // Performing unary op
        Ok(Self::perform_unary_op(span, value, op))
    }

    /// Evaluates variable expression
    pub(crate) fn eval_variable(&self, span: &Span, name: &str) -> Flow<Value> {
        // Current realm
        if let Some(value) = self.realm.borrow().lookup(name) {
            Ok(value)
        }
        // Builtins realm
        else if let Some(value) = self.builtins.env.borrow().lookup(name) {
            Ok(value)
        }
        // Otherwise, raising error
        else {
            bail!(RuntimeError::UndefinedVariable {
                name: name.to_string(),
                src: span.0.clone(),
                span: span.1.clone().into(),
            })
        }
    }

    /// Performs field access
    pub(crate) fn access_field(span: &Span, name: &str, container: Value) -> Flow<Value> {
        // Matching container
        match container {
            // Module field access
            Value::Module(m) => match m.borrow().env.borrow().lookup(name) {
                Some(it) => Ok(it.clone()),
                None => bail!(RuntimeError::UndefinedField {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    name: name.to_string()
                }),
            },
            // Instance field access
            Value::Instance(i) => match i.borrow().fields.get(name) {
                Some(it) => Ok(it.clone()),
                None => bail!(RuntimeError::UndefinedField {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    name: name.to_string()
                }),
            },
            // Enum field access
            Value::Enum(e) => match e.variants.iter().position(|v| v == name) {
                Some(idx) => Ok(Value::Int(idx as i64)),
                None => bail!(RuntimeError::UndefinedField {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    name: name.to_string()
                }),
            },
            // Otherwise, raising error
            value => bail!(RuntimeError::CouldNotResolveFields {
                src: span.0.clone(),
                span: span.1.clone().into(),
                value
            }),
        }
    }

    /// Evaluates field expression
    pub(crate) fn eval_field(
        &mut self,
        span: &Span,
        name: &str,
        container: &Expression,
    ) -> Flow<Value> {
        // Evaluating container
        let container = self.eval(container)?;

        // Accessing field
        Self::access_field(span, name, container)
    }

    /// Checks params and arguments arity
    fn check_arity(&self, span: &Span, params: usize, args: usize) {
        // Checking arity
        if params != args {
            // Raising error on arity missmatch
            bail!(RuntimeError::IncorrectArity {
                src: span.0.clone(),
                span: span.1.clone().into(),
                params,
                args
            })
        }
    }

    /// Prepares instance fields map
    fn prepare_instance_fields(
        &self,
        instance: &MutRef<Instance>,
        class: Ref<Class>,
    ) -> HashMap<String, Value> {
        // Iterating over class methods
        class
            .methods
            .clone()
            .into_iter()
            .map(|it| {
                (
                    it.0,
                    // Creating bound method for each
                    Value::Callable(Callable::Bound(Ref::new(Bound {
                        method: it.1,
                        // Field belongs to fresh instance
                        belongs_to: instance.clone(),
                    }))),
                )
            })
            .collect()
    }

    /// Creates instance of the class
    fn create_instance(&mut self, class: Ref<Class>) -> MutRef<Instance> {
        // Creating instance
        let instance = MutRef::new(RefCell::new(Instance {
            type_of: class.clone(),
            fields: HashMap::new(),
        }));

        // Preparing instance fields
        let fields = self.prepare_instance_fields(&instance, class);

        // Setting new fields for instance
        instance.borrow_mut().fields = fields;
        instance
    }

    /// Evaluates arguments
    fn eval_args(&mut self, args: &[Expression]) -> Flow<Vec<Value>> {
        let args: Result<Vec<Value>, ControlFlow> =
            args.iter().map(|expr| self.eval(expr)).collect();
        args
    }

    /// Calls closure
    pub(crate) fn call_closure(
        &mut self,
        span: &Span,
        args: Vec<Value>,
        closure: Ref<Closure>,
    ) -> Flow<Value> {
        // Checking arity
        self.check_arity(span, closure.function.params.len(), args.len());

        // Pushing realm
        let previous = self.realm.clone();
        self.realm = RealmRef::new(RefCell::new(Realm::new(closure.realm.clone())));

        // Defining arguments
        closure
            .function
            .params
            .iter()
            .zip(args)
            .for_each(|(p, a)| self.realm.borrow_mut().define(p, a));

        // Executing closure block
        let result = {
            match self.exec_block(&closure.function.block, false) {
                Ok(_) => Value::Null,
                Err(flow) => match flow {
                    ControlFlow::Return(value) => value,
                    _ => bug!("control flow leak."),
                },
            }
        };

        // Popping realm
        self.realm = previous;

        // Done!
        Ok(result)
    }

    /// Calls native function
    pub(crate) fn call_native(
        &mut self,
        span: &Span,
        args: Vec<Value>,
        native: Ref<Native>,
    ) -> Flow<Value> {
        // Checking arity
        self.check_arity(span, native.arity, args.len());

        // Pushing realm
        let previous = self.realm.clone();
        self.realm = RealmRef::new(RefCell::new(Realm::default()));

        // Executing
        let result = (*native.function)(self, span, args);

        // Popping realm
        self.realm = previous;

        // Done!
        Ok(result)
    }

    /// Calls type and creates instance
    pub(crate) fn call_class(
        &mut self,
        span: &Span,
        args: Vec<Value>,
        ty: Ref<Class>,
    ) -> Flow<Value> {
        // Creating instance
        let instance = self.create_instance(ty);

        // If `init` exists and is a bound method, call it
        if let Some(Value::Callable(Callable::Bound(bound))) = {
            // Temp borrow
            let borrow = instance.borrow();
            borrow.fields.get("init").cloned()
        } {
            // Calling bound method, if found
            self.call_bound_method(span, args, bound)?;
        } else {
            // Either no init or not a bound method -> check arity 0
            self.check_arity(span, 0, args.len());
        }

        // Done!
        Ok(Value::Instance(instance))
    }

    /// Calls callable
    pub(crate) fn call(
        &mut self,
        span: &Span,
        args: Vec<Value>,
        callable: Callable,
    ) -> Flow<Value> {
        match callable {
            Callable::Closure(closure) => self.call_closure(span, args, closure),
            Callable::Bound(bound) => self.call_bound_method(span, args, bound),
            Callable::Native(native) => self.call_native(span, args, native),
        }
    }

    /// Calls bound method
    fn call_bound_method(
        &mut self,
        span: &Span,
        mut args: Vec<Value>,
        bound: Ref<Bound>,
    ) -> Flow<Value> {
        // Inserting `self` parameter
        args.insert(0, Value::Instance(bound.belongs_to.clone()));

        // Bound closure
        match &bound.method {
            Method::Native(native) => self.call_native(span, args, native.clone()),
            Method::Closure(closure) => self.call_closure(span, args, closure.clone()),
        }
    }

    /// Evaluates call expression
    fn eval_call(&mut self, span: &Span, args: &[Expression], what: &Expression) -> Flow<Value> {
        // Evaluating arguments
        let args = self.eval_args(args)?;

        // Evaluating callee
        let value = self.eval(what)?;
        match value {
            // Calling
            Value::Callable(callable) => self.call(span, args, callable),
            Value::Class(ty) => self.call_class(span, args, ty),
            _ => bail!(RuntimeError::CouldNotCall {
                src: span.0.clone(),
                span: span.1.clone().into(),
                value
            }),
        }
    }

    /// Evaluates list expression
    fn eval_list(&mut self, span: &Span, list: &[Expression]) -> Flow<Value> {
        // Evaluating values before accessing list
        let mut values = Vec::new();
        for expr in list {
            let val = self.eval(expr)?;
            values.push(val);
        }

        // Calling list constructor
        let list_value = {
            let list_value = self
                .builtins
                .env
                .borrow()
                .lookup("List")
                .unwrap_or_else(|| bug!("no builtin `List` found"));

            match list_value {
                Value::Class(t) => match self.call_class(span, Vec::new(), t)? {
                    Value::Instance(instance) => instance,
                    _ => unreachable!(),
                },
                _ => bug!("builtin `List` is not a class"),
            }
        };

        // Setting new vector
        list_value.borrow_mut().fields.insert(
            "$internal".to_string(),
            Value::Any(MutRef::new(RefCell::new(values))),
        );

        Ok(Value::Instance(list_value))
    }

    /// Evaluates dict expression
    fn eval_dict(&mut self, span: &Span, dict: &[(Expression, Expression)]) -> Flow<Value> {
        // Note: Value implementation of Hash doesn’t
        // access any of the interior mutable types, so it's a false positive
        #[allow(clippy::mutable_key_type)]
        // Evaluating values before accessing dict
        let mut values_map = HashMap::new();
        for (a, b) in dict {
            let key = self.eval(a)?;
            let val = self.eval(b)?;
            values_map.insert(key, val);
        }

        // Calling dict constructor
        let dict_value = {
            let dict_value = self
                .builtins
                .env
                .borrow()
                .lookup("Dict")
                .unwrap_or_else(|| bug!("no builtin `Dict` found"));

            match dict_value {
                Value::Class(t) => match self.call_class(span, Vec::new(), t)? {
                    Value::Instance(instance) => instance,
                    _ => unreachable!(),
                },
                _ => bug!("builtin `Dict` is not a class"),
            }
        };

        // Setting new map
        dict_value.borrow_mut().fields.insert(
            "$internal".to_string(),
            Value::Any(MutRef::new(RefCell::new(values_map))),
        );

        Ok(Value::Instance(dict_value))
    }

    /// Evaluates range expression
    fn eval_range(
        &mut self,
        span: &Span,
        lhs: &Expression,
        rhs: &Expression,
        includes_end: bool,
    ) -> Flow<Value> {
        // Evaluating lhs and rhs before accessing list
        let lhs = self.eval(lhs)?;
        let rhs = self.eval(rhs)?;

        // Creating vector of values
        let values = match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => {
                let range = if includes_end { a..b + 1 } else { a..b };
                range.map(Value::Int).collect::<Vec<Value>>()
            }
            (lhs, rhs) => bail!(RuntimeError::InvalidRange {
                src: span.0.clone(),
                span: span.1.clone().into(),
                lhs,
                rhs,
            }),
        };

        // Calling list constructor
        let list_value = {
            let list_value = self
                .builtins
                .env
                .borrow()
                .lookup("List")
                .unwrap_or_else(|| bug!("no builtin `List` found"));

            match list_value {
                Value::Class(t) => match self.call_class(span, Vec::new(), t)? {
                    Value::Instance(instance) => instance,
                    _ => unreachable!(),
                },
                _ => bug!("builtin `List` is not a class"),
            }
        };

        // Setting new vector
        list_value.borrow_mut().fields.insert(
            "$internal".to_string(),
            Value::Any(MutRef::new(RefCell::new(values))),
        );

        Ok(Value::Instance(list_value))
    }

    /// Evaluates lambda expression
    fn eval_anon_fn(&mut self, params: &[String], block: &Block) -> Flow<Value> {
        Ok(Value::Callable(Callable::Closure(Ref::new(Closure {
            function: Ref::new(Function {
                params: params.to_owned(),
                block: block.clone(),
            }),
            realm: self.realm.clone(),
        }))))
    }

    /// Evaluates expression
    pub fn eval(&mut self, expr: &Expression) -> Flow<Value> {
        // Matching expression
        match expr {
            Expression::Lit { lit, .. } => self.eval_lit(lit),
            Expression::Bin {
                span,
                op,
                lhs: left,
                rhs: right,
            } => self.eval_bin(span, *op, left, right),
            Expression::Unary { span, op, value } => self.eval_unary(span, *op, value),
            Expression::Variable { span, name } => self.eval_variable(span, name),
            Expression::Field {
                span,
                name,
                container,
            } => self.eval_field(span, name, container),
            Expression::Call { span, args, what } => self.eval_call(span, args, what),
            Expression::List { span, list } => self.eval_list(span, list),
            Expression::Dict { span, dict } => self.eval_dict(span, dict),
            Expression::Range {
                span,
                lhs,
                rhs,
                includes_end,
            } => self.eval_range(span, lhs, rhs, *includes_end),
            Expression::Fun { params, block, .. } => self.eval_anon_fn(params, block),
        }
    }

    /// Is truthy helper
    pub(crate) fn is_truthy(&self, span: &Span, value: &Value) -> bool {
        if let Value::Bool(bool) = value {
            *bool
        } else {
            bail!(RuntimeError::ExpectedBool {
                value: value.clone(),
                src: span.0.clone(),
                span: span.1.clone().into()
            })
        }
    }

    /// Is impls helper
    pub(crate) fn is_impls(span: &Span, val: Value, trt: Value) -> bool {
        match (val, trt) {
            (Value::Instance(a), Value::Trait(b)) => {
                // Iterating over trait functions
                for func in &b.functions {
                    // Checking implementation
                    match a.borrow().fields.get(&func.name) {
                        Some(Value::Callable(callable)) => {
                            let arity = match callable {
                                Callable::Closure(closure) => closure.function.params.len(),
                                Callable::Bound(bound) => match &bound.method {
                                    Method::Native(native) => native.arity,
                                    Method::Closure(closure) => closure.function.params.len(),
                                },
                                Callable::Native(native) => native.arity,
                            };
                            if arity != func.arity {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                }
                true
            }
            (_, Value::Trait(_)) => false,
            (a, b) => {
                bail!(RuntimeError::InvalidBinOp {
                    op: BinOp::Impls,
                    a,
                    b,
                    src: span.0.clone(),
                    span: span.1.clone().into()
                });
            }
        }
    }
}
