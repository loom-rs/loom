/// Imports
use crate::{
    error::RuntimeError,
    interpreter::Interpreter,
    refs::{EnvRef, MutRef, Ref},
    rt::{
        env::Environment,
        flow::{ControlFlow, Flow},
        value::{Bound, Callable, Closure, Function, Instance, Method, Native, Type, Value},
    },
};
use std::{cell::RefCell, collections::HashMap};
use tick_ast::{
    atom::{BinaryOp, Lit, UnaryOp},
    expr::Expression,
    stmt::Block,
};
use tick_common::{bail, bug};
use tick_lex::token::Span;

/// Implementation
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

    /// Performs binary operation over values
    pub(crate) fn perform_binary_op(
        &self,
        span: &Span,
        left: Value,
        right: Value,
        op: &BinaryOp,
    ) -> Value {
        // Invalid binary op
        let invalid_bin_op = || {
            bail!(RuntimeError::InvalidBinaryOp {
                op: op.clone(),
                a: left.clone(),
                b: right.clone(),
                src: span.0.clone(),
                span: span.1.clone().into()
            });
        };

        // Matching binary operator
        match (left.clone(), right.clone()) {
            (Value::Bool(a), Value::Bool(b)) => match op {
                BinaryOp::And => Value::Bool(a && b),
                BinaryOp::Or => Value::Bool(a || b),
                BinaryOp::Gt => Value::Bool(a > b),
                BinaryOp::Ge => Value::Bool(a >= b),
                BinaryOp::Lt => Value::Bool(a < b),
                BinaryOp::Le => Value::Bool(a <= b),
                BinaryOp::Eq => Value::Bool(a == b),
                BinaryOp::Ne => Value::Bool(a != b),
                BinaryOp::BitAnd => Value::Bool(a & b),
                BinaryOp::BitOr => Value::Bool(a | b),
                BinaryOp::Xor => Value::Bool(a ^ b),
                _ => invalid_bin_op(),
            },
            (Value::Int(a), Value::Int(b)) => match op {
                BinaryOp::Gt => Value::Bool(a > b),
                BinaryOp::Ge => Value::Bool(a >= b),
                BinaryOp::Lt => Value::Bool(a < b),
                BinaryOp::Le => Value::Bool(a <= b),
                BinaryOp::Eq => Value::Bool(a == b),
                BinaryOp::Ne => Value::Bool(a != b),
                BinaryOp::Add => Value::Int(a + b),
                BinaryOp::Sub => Value::Int(a - b),
                BinaryOp::Mul => Value::Int(a * b),
                BinaryOp::Div => Value::Int(a / b),
                BinaryOp::Mod => Value::Int(a % b),
                BinaryOp::Xor => Value::Int(a ^ b),
                BinaryOp::BitAnd => Value::Int(a & b),
                BinaryOp::BitOr => Value::Int(a | b),
                _ => invalid_bin_op(),
            },
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => match op {
                BinaryOp::Gt => Value::Bool((a as f64) > b),
                BinaryOp::Ge => Value::Bool((a as f64) >= b),
                BinaryOp::Lt => Value::Bool((a as f64) < b),
                BinaryOp::Le => Value::Bool((a as f64) <= b),
                BinaryOp::Eq => Value::Bool((a as f64) == b),
                BinaryOp::Ne => Value::Bool((a as f64) != b),
                BinaryOp::Add => Value::Float((a as f64) + b),
                BinaryOp::Sub => Value::Float((a as f64) - b),
                BinaryOp::Mul => Value::Float((a as f64) * b),
                BinaryOp::Div => Value::Float((a as f64) / b),
                BinaryOp::Mod => Value::Float((a as f64) % b),
                _ => invalid_bin_op(),
            },
            (a, Value::String(b)) => Value::String(format!("{a}{b}")),
            (Value::String(a), b) => Value::String(format!("{a}{b}")),
            (Value::Float(a), Value::Float(b)) => match op {
                BinaryOp::Gt => Value::Bool(a > b),
                BinaryOp::Ge => Value::Bool(a >= b),
                BinaryOp::Lt => Value::Bool(a < b),
                BinaryOp::Le => Value::Bool(a <= b),
                BinaryOp::Eq => Value::Bool(a == b),
                BinaryOp::Ne => Value::Bool(a != b),
                BinaryOp::Add => Value::Float(a + b),
                BinaryOp::Sub => Value::Float(a - b),
                BinaryOp::Mul => Value::Float(a * b),
                BinaryOp::Div => Value::Float(a / b),
                BinaryOp::Mod => Value::Float(a % b),
                _ => invalid_bin_op(),
            },
            (a, b) => match op {
                BinaryOp::Eq => Value::Bool(a == b),
                BinaryOp::Ne => Value::Bool(a != b),
                _ => invalid_bin_op(),
            },
        }
    }

    /// Evaluates binary expression
    pub(crate) fn eval_binary(
        &mut self,
        span: &Span,
        op: &BinaryOp,
        left: &Expression,
        right: &Expression,
    ) -> Flow<Value> {
        // Evaluating lhs and rhs
        let left = self.eval(left)?;
        let right = self.eval(right)?;

        // Performing bin op
        Ok(self.perform_binary_op(span, left, right, op))
    }

    /// Performs unary operation over values
    pub(crate) fn perform_unary_op(&self, span: &Span, value: Value, op: &UnaryOp) -> Value {
        // Invalid unary op
        let invalid_unary_op = || {
            bail!(RuntimeError::InvalidUnaryOp {
                op: op.clone(),
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
        op: &UnaryOp,
        value: &Expression,
    ) -> Flow<Value> {
        // Evaluating value
        let value = self.eval(value)?;

        // Performing unary op
        Ok(self.perform_unary_op(span, value, op))
    }

    /// Evaluates variable expression
    pub(crate) fn eval_variable(&self, span: &Span, name: &str) -> Flow<Value> {
        // Current environment
        if let Some(value) = self.env.borrow().lookup(name) {
            Ok(value)
        }
        // Builtins environment
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

    /// Evaluates field expression
    pub(crate) fn eval_field(
        &mut self,
        span: &Span,
        name: &str,
        container: &Expression,
    ) -> Flow<Value> {
        // Evaluating container
        let container = self.eval(container)?;
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
        ty: Ref<Type>,
    ) -> HashMap<String, Value> {
        // Iterating over type methods
        ty.methods
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

    /// Creates instance of the type
    fn create_instance(&mut self, ty: Ref<Type>) -> MutRef<Instance> {
        // Creating instance
        let instance = MutRef::new(RefCell::new(Instance {
            type_of: ty.clone(),
            fields: HashMap::new(),
        }));
        // Preparing instance fields
        let fields = self.prepare_instance_fields(&instance, ty);
        // Setting new fields for instance
        instance.borrow_mut().fields = fields;
        instance
    }

    /// Evaluates arguments
    fn eval_args(&mut self, args: &Vec<Expression>) -> Flow<Vec<Value>> {
        let args: Result<Vec<Value>, ControlFlow> =
            args.into_iter().map(|expr| self.eval(expr)).collect();
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

        // Pushing environment
        let previous = self.env.clone();
        self.env = EnvRef::new(RefCell::new(Environment::new(closure.environment.clone())));

        // Defining arguments
        closure
            .function
            .params
            .iter()
            .zip(args)
            .for_each(|(p, a)| self.env.borrow_mut().define(span, p, a));

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

        // Popping environment
        self.env = previous;

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

        // Pushing environment
        let previous = self.env.clone();
        self.env = EnvRef::new(RefCell::new(Environment::default()));

        // Executing
        let result = (*native.function)(self, span, args);

        // Popping environment
        self.env = previous;

        // Done!
        Ok(result)
    }

    /// Calls type and creates instance
    pub(crate) fn call_type(
        &mut self,
        span: &Span,
        args: Vec<Value>,
        ty: Ref<Type>,
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
    fn eval_call(&mut self, span: &Span, args: &Vec<Expression>, what: &Expression) -> Flow<Value> {
        // Evaluating arguments
        let args = self.eval_args(args)?;

        // Evaluating callee
        let value = self.eval(what)?;
        match value {
            // Calling
            Value::Callable(callable) => match callable {
                Callable::Closure(closure) => self.call_closure(span, args, closure),
                Callable::Bound(bound) => self.call_bound_method(span, args, bound),
                Callable::Native(native) => self.call_native(span, args, native),
            },
            Value::Type(ty) => self.call_type(span, args, ty),
            _ => bail!(RuntimeError::CouldNotCall {
                src: span.0.clone(),
                span: span.1.clone().into(),
                value
            }),
        }
    }

    /// Evaluates list expression
    fn eval_list(&mut self, span: &Span, list: &Vec<Expression>) -> Flow<Value> {
        // Evaluating values before accessing list
        let values = list
            .iter()
            .map(|expr| self.eval(expr))
            .collect::<Flow<Vec<Value>>>()?;

        // Calling list constructor
        let list_value = {
            let list_value = self
                .builtins
                .env
                .borrow()
                .lookup("List")
                .unwrap_or_else(|| bug!("no builtin `List` found"));

            match list_value {
                Value::Type(t) => match self.call_type(span, Vec::new(), t)? {
                    Value::Instance(instance) => instance,
                    _ => unreachable!(),
                },
                _ => bug!("builtin `List` is not a type"),
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
    fn eval_anon_fn(&mut self, params: &Vec<String>, block: &Block) -> Flow<Value> {
        Ok(Value::Callable(Callable::Closure(Ref::new(Closure {
            function: Ref::new(Function {
                params: params.clone(),
                block: block.clone(),
            }),
            environment: self.env.clone(),
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
                left,
                right,
            } => self.eval_binary(span, op, left, right),
            Expression::Unary { span, op, value } => self.eval_unary(span, op, value),
            Expression::Variable { span, name } => self.eval_variable(span, name),
            Expression::Field {
                span,
                name,
                container,
            } => self.eval_field(span, name, container),
            Expression::Call { span, args, what } => self.eval_call(span, args, what),
            Expression::List { span, list } => self.eval_list(span, list),
            Expression::Fn { params, block, .. } => self.eval_anon_fn(params, block),
        }
    }

    /// Is truthy helper
    pub(crate) fn is_truthy(&self, span: &Span, value: &Value) -> bool {
        if let Value::Bool(bool) = value {
            bool.clone()
        } else {
            bail!(RuntimeError::ExpectedBool {
                value: value.clone(),
                src: span.0.clone(),
                span: span.1.clone().into()
            })
        }
    }
}
