/// Imports
use crate::{
    VirtualMachine,
    errors::RuntimeError,
    frame::Scope,
    ops::{Label, Opcode},
    refs::{MutRef, Ref},
    value::{
        Bound, Callable, Class, Closure, Function, Instance, Method, Module, Native, Trait,
        TraitFunction,
        Value::{self},
    },
};
use common::{bail, bug, span::Span};
use std::{cell::RefCell, collections::HashMap};

/// Implementation of the VM
impl<'io, 'reg> VirtualMachine<'io, 'reg> {
    /// Executes push op
    fn op_push(&mut self, value: Value) {
        self.frame_mut().push(value);
    }

    /// Executes pop op
    fn op_pop(&mut self) {
        self.frame_mut().pop();
    }

    /// Executes dup op
    fn op_dup(&mut self) {
        let frame = self.frame_mut();
        let value = frame.pop();

        frame.push(value.clone());
        frame.push(value);
    }

    /// Executes add op
    fn op_add(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 + b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a + b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "+".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes sub op
    fn op_sub(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
            (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 - b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a - b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "-".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes mul op
    fn op_mul(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 * b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a * b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "*".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes div op
    fn op_div(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) if b > 0 => Value::Int(a / b),
            (Value::Int(a), Value::Float(b)) if b > 0.0 => Value::Float(a as f64 / b),
            (Value::Float(a), Value::Int(b)) if b > 0 => Value::Float(a / b as f64),
            (Value::Float(a), Value::Float(b)) if b > 0.0 => Value::Float(a / b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "/".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes rem op
    fn op_rem(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
            (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 % b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a % b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a % b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "%".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes gt op
    fn op_gt(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
            (Value::Int(a), Value::Float(b)) => Value::Bool(a as f64 > b),
            (Value::Float(a), Value::Int(b)) => Value::Bool(a > b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: ">".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes ge op
    fn op_ge(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a >= b),
            (Value::Int(a), Value::Float(b)) => Value::Bool(a as f64 >= b),
            (Value::Float(a), Value::Int(b)) => Value::Bool(a >= b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: ">=".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes lt op
    fn op_lt(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
            (Value::Int(a), Value::Float(b)) => Value::Bool((a as f64) < b),
            (Value::Float(a), Value::Int(b)) => Value::Bool(a < b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "<".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes le op
    fn op_le(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
            (Value::Int(a), Value::Float(b)) => Value::Bool(a as f64 <= b),
            (Value::Float(a), Value::Int(b)) => Value::Bool(a <= b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "<=".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes eq op
    fn op_eq(&mut self) {
        let frame = self.frame_mut();

        let rhs = frame.pop();
        let lhs = frame.pop();

        frame.push(Value::Bool(lhs == rhs));
    }

    /// Executes ne op
    fn op_ne(&mut self) {
        let frame = self.frame_mut();

        let rhs = frame.pop();
        let lhs = frame.pop();

        frame.push(Value::Bool(lhs != rhs));
    }

    /// Executes and op
    fn op_and(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "&&".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes or op
    fn op_or(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "||".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes xor op
    fn op_xor(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a ^ b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a ^ b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "^".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes band op
    fn op_band(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a & b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a & b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "&".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes bor op
    fn op_bor(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = match (lhs, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a | b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a | b),
            (a, b) => bail!(RuntimeError::InvalidBinOp {
                op: "|".into(),
                a,
                b,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Does class impls trait
    fn is_impls(span: &Span, lhs: Value, rhs: Value) -> bool {
        // Mathing lhs and rhs
        match (lhs, rhs) {
            // If lhs is an instance and rhs is a trait
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
                    op: ">:".into(),
                    a,
                    b,
                    src: span.0.clone(),
                    span: span.1.clone().into()
                });
            }
        }
    }

    /// Executes impls op
    fn op_impls(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = Self::is_impls(&span, lhs, rhs);
        frame.push(Value::Bool(result))
    }

    /// Executes not impls op
    fn op_not_impls(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();

        let rhs = frame.pop();
        let lhs = frame.pop();

        let result = Self::is_impls(&span, lhs, rhs);
        frame.push(Value::Bool(!result))
    }

    /// Executes neg op
    fn op_neg(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();
        let value = frame.pop();

        let result = match value {
            Value::Int(a) => Value::Int(-a),
            Value::Float(a) => Value::Float(-a),
            value => bail!(RuntimeError::InvalidUnaryOp {
                op: "-".into(),
                value,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes bang op
    fn op_bang(&mut self) {
        let frame = self.frame_mut();
        let span = frame.span();
        let value = frame.pop();

        let result = match value {
            Value::Bool(a) => Value::Bool(!a),
            value => bail!(RuntimeError::InvalidUnaryOp {
                op: "!".into(),
                value,
                src: span.0,
                span: span.1.into(),
            }),
        };

        frame.push(result);
    }

    /// Executes jump op
    fn op_jump(&mut self, label: Label) {
        let frame = self.frame_mut();
        let pc = frame.pc_of_label(label);

        frame.jump(pc);
    }

    /// Executes jump if true op
    fn op_jump_if_true(&mut self, label: Label) {
        let frame = self.frame_mut();
        let value = frame.pop();

        let result = if let Value::Bool(a) = value {
            a
        } else {
            bug!("jump if with non-bool value")
        };

        if result {
            let pc = frame.pc_of_label(label);
            frame.jump(pc);
        }
    }

    /// Executes jump if false op
    fn op_jump_if_false(&mut self, label: Label) {
        let frame = self.frame_mut();
        let value = frame.pop();

        let result = if let Value::Bool(a) = value {
            a
        } else {
            bug!("jump if with non-bool value")
        };

        if !result {
            let pc = frame.pc_of_label(label);
            frame.jump(pc);
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

    /// Calls closure
    pub fn call_closure(&mut self, span: &Span, closure: Ref<Closure>, args: Vec<Value>) -> Value {
        // Checking arity
        self.check_arity(span, closure.function.params.len(), args.len());

        // Pushing frame with enclosing scope
        self.push_with_enclosing(closure.function.chunk.clone(), closure.scope.clone());

        // Defining arguments
        closure
            .function
            .params
            .iter()
            .zip(args)
            .for_each(|(p, a)| self.frame_mut().scope.borrow_mut().insert(p, a));

        // Running loop
        self.exec();

        // Popping result
        let result = self.frame_mut().pop();

        // Popping frame
        self.pop();

        // Done!
        result
    }

    /// Calls native function
    pub fn call_native(&mut self, span: &Span, native: Ref<Native>, args: Vec<Value>) -> Value {
        // Checking arity
        self.check_arity(span, native.arity, args.len());

        // Executing native function
        (*native.function)(self, span, args)
    }

    /// Calls bound method
    pub fn call_bound_method(
        &mut self,
        span: &Span,
        bound: Ref<Bound>,
        mut args: Vec<Value>,
    ) -> Value {
        // Inserting `self` parameter
        args.insert(0, Value::Instance(bound.belongs_to.clone()));

        // Bound closure
        match &bound.method {
            Method::Native(native) => self.call_native(span, native.clone(), args),
            Method::Closure(closure) => self.call_closure(span, closure.clone(), args),
        }
    }

    /// Calls type and creates instance
    pub fn call_class(&mut self, span: &Span, class: Ref<Class>, args: Vec<Value>) -> Value {
        // Creating instance
        let instance = self.create_instance(class);

        // If `init` exists and it's a bound method, calling it
        if let Some(Value::Callable(Callable::Bound(bound))) = {
            // Temp borrow
            let borrow = instance.borrow();
            borrow.fields.get("init").cloned()
        } {
            // Calling bound method, if found
            self.call_bound_method(span, bound, args);
        } else {
            // Either no init or not a bound method -> check arity 0
            self.check_arity(span, 0, args.len());
        }

        // Returning created instance
        Value::Instance(instance)
    }

    /// Calls callable
    pub fn call(&mut self, span: &Span, callable: Callable, args: Vec<Value>) -> Value {
        match callable {
            Callable::Closure(closure) => self.call_closure(span, closure, args),
            Callable::Bound(bound) => self.call_bound_method(span, bound, args),
            Callable::Native(native) => self.call_native(span, native, args),
        }
    }

    /// Executes call op
    fn op_call(&mut self, arity: usize) {
        let frame = self.frame_mut();
        let span = frame.span();

        // Popping arguments
        let mut args = Vec::new();
        for _ in 0..arity {
            args.insert(0, frame.pop());
        }

        // Popping value we need to call
        let value = frame.pop();

        // Calling callable with args
        let result = match value {
            // Calling
            Value::Callable(callable) => self.call(&span, callable, args),
            Value::Class(ty) => self.call_class(&span, ty, args),
            _ => bail!(RuntimeError::CouldNotCall {
                src: span.0,
                span: span.1.into(),
                value
            }),
        };

        // Pushing result onto the stack
        self.frame_mut().push(result);
    }

    /// Performs field access
    pub(crate) fn access_field(span: &Span, name: &str, container: Value) -> Value {
        // Matching container
        match container {
            // Module field access
            Value::Module(m) => match m.borrow().scope.borrow().lookup(name) {
                Some(it) => it.clone(),
                None => bail!(RuntimeError::UndefinedField {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    name: name.to_string()
                }),
            },
            // Instance field access
            Value::Instance(i) => match i.borrow().fields.get(name) {
                Some(it) => it.clone(),
                None => bail!(RuntimeError::UndefinedField {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    name: name.to_string()
                }),
            },
            // Enum field access
            Value::Enum(e) => match e.variants.iter().position(|v| v == name) {
                Some(idx) => Value::Int(idx as i64),
                None => bail!(RuntimeError::UndefinedField {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    name: name.to_string()
                }),
            },
            // Otherwise, raising error
            value => bail!(RuntimeError::CouldNotLookupField {
                src: span.0.clone(),
                span: span.1.clone().into(),
                value
            }),
        }
    }

    /// Performs field set
    pub(crate) fn set_field(span: &Span, name: &str, container: Value, value: Value) {
        // Matching container
        match container {
            // Module field access
            Value::Module(m) => {
                if m.borrow().scope.borrow().exists(name) {
                    m.borrow().scope.borrow_mut().insert(name, value);
                } else {
                    bail!(RuntimeError::UndefinedField {
                        src: span.0.clone(),
                        span: span.1.clone().into(),
                        name: name.to_string()
                    })
                }
            }
            // Instance field access
            Value::Instance(i) => {
                if i.borrow().fields.contains_key(name) {
                    i.borrow_mut().fields.insert(name.into(), value);
                } else {
                    bail!(RuntimeError::UndefinedField {
                        src: span.0.clone(),
                        span: span.1.clone().into(),
                        name: name.to_string()
                    })
                }
            }
            // Otherwise, raising error
            value => bail!(RuntimeError::CouldNotAssignField {
                src: span.0.clone(),
                span: span.1.clone().into(),
                value
            }),
        }
    }

    /// Performs field define
    pub(crate) fn define_field(span: &Span, name: &str, container: Value, value: Value) {
        // Matching container
        match container {
            // Module field access
            Value::Module(m) => {
                m.borrow().scope.borrow_mut().insert(name, value);
            }
            // Instance field access
            Value::Instance(i) => {
                i.borrow_mut().fields.insert(name.into(), value);
            }
            // Otherwise, raising error
            value => bail!(RuntimeError::CouldNotDefineField {
                src: span.0.clone(),
                span: span.1.clone().into(),
                value
            }),
        }
    }

    /// Executes field op
    fn op_load_field(&mut self, field: String) {
        let frame = self.frame_mut();
        let span = frame.span();
        let container = frame.pop();
        frame.push(Self::access_field(&span, &field, container))
    }

    /// Executes store field op
    fn op_store_field(&mut self, field: String) {
        let frame = self.frame_mut();
        let span = frame.span();

        let value = frame.pop();
        let container = frame.pop();

        Self::set_field(&span, &field, container, value)
    }

    /// Executes define field op
    fn op_define_field(&mut self, field: String) {
        let frame = self.frame_mut();
        let span = frame.span();

        let value = frame.pop();
        let container = frame.pop();

        Self::define_field(&span, &field, container, value)
    }

    /// Executes load op
    fn op_load(&mut self, name: String) {
        let value = if let Some(value) = self.frame_mut().scope.borrow().lookup(&name) {
            value
        } else if let Some(value) = self.builtins.borrow().lookup(&name) {
            value
        } else {
            let span = self.frame().span();
            bail!(RuntimeError::UndefinedVariable {
                name,
                src: span.0,
                span: span.1.into()
            })
        };

        self.frame_mut().push(value);
    }

    /// Executes store op
    fn op_store(&mut self, name: String) {
        let frame = self.frame_mut();
        let span = frame.span();
        let value = frame.pop();

        let mut scope = frame.scope.borrow_mut();
        if scope.exists(&name) {
            scope.insert(&name, value);
        } else {
            bail!(RuntimeError::UndefinedVariable {
                name,
                src: span.0,
                span: span.1.into()
            })
        }
    }

    /// Executes define op
    fn op_define(&mut self, name: String) {
        let frame = self.frame_mut();
        let value = frame.pop();
        let mut scope = frame.scope.borrow_mut();
        scope.insert(&name, value);
    }

    /// Executes make closure op
    fn op_make_closure(&mut self, function: Ref<Function>) {
        let callable = Value::Callable(Callable::Closure(Ref::new(Closure {
            function,
            scope: self.frame_mut().scope.clone(),
        })));
        self.frame_mut().push(callable);
    }

    /// Executes make class op
    fn op_make_class(&mut self, name: String, methods: HashMap<String, Ref<Function>>) {
        // Preparing class
        let scope = self.frame_mut().scope.clone();
        let class = Ref::new(Class {
            name,
            methods: methods
                .into_iter()
                .map(|(name, function)| {
                    (
                        name,
                        Method::Closure(Ref::new(Closure {
                            function,
                            scope: scope.clone(),
                        })),
                    )
                })
                .collect(),
        });

        // Pushing class onto stack
        self.frame_mut().push(Value::Class(class));
    }

    /// Executes make trait op
    fn op_make_trait(&mut self, name: String, functions: Vec<TraitFunction>) {
        self.frame_mut()
            .push(Value::Trait(Ref::new(Trait { name, functions })));
    }

    /// Executes import op
    fn op_import(&mut self, path: String) {
        let span = self.frame().span();

        // Resolving module
        let (id, chunk) = self.modules.resolve(span, &path);

        // Preparing module scope
        let scope = MutRef::new(RefCell::new(Scope::default()));

        // Inserting module to registry
        let module = MutRef::new(RefCell::new(Module {
            scope: self.frame().scope.clone(),
        }));
        self.modules.insert(&id, module.clone());

        // Pushing frame
        self.push_with_scope(chunk, scope);

        // Executing module
        self.exec();

        // Pushing module onto stack
        self.frame_mut().push(Value::Module(module));
    }

    /// Runs vm execution loop
    pub fn exec(&mut self) {
        while let Some(op) = self.frame().op() {
            match op {
                // Does nothing
                Opcode::Nop => {}
                // Pushes value onto the stack
                Opcode::Push(value) => self.op_push(value),
                // Pops value from the stack
                Opcode::Pop => self.op_pop(),
                // Dups value on the stack
                Opcode::Dup => self.op_dup(),
                // Arithmetical operations
                Opcode::Add => self.op_add(),
                Opcode::Sub => self.op_sub(),
                Opcode::Mul => self.op_mul(),
                Opcode::Div => self.op_div(),
                Opcode::Rem => self.op_rem(),
                // Compare operations
                Opcode::Gt => self.op_gt(),
                Opcode::Ge => self.op_ge(),
                Opcode::Lt => self.op_lt(),
                Opcode::Le => self.op_le(),
                // Equality operations
                Opcode::Eq => self.op_eq(),
                Opcode::Ne => self.op_ne(),
                // Logical operations
                Opcode::And => self.op_and(),
                Opcode::Or => self.op_or(),
                Opcode::Xor => self.op_xor(),
                Opcode::Band => self.op_band(),
                Opcode::Bor => self.op_bor(),
                // Trait operations
                Opcode::Impls => self.op_impls(),
                Opcode::NotImpls => self.op_not_impls(),
                // Unary operations
                Opcode::Neg => self.op_neg(),
                Opcode::Bang => self.op_bang(),
                // Jump operations
                Opcode::Jump(label) => self.op_jump(label),
                Opcode::JumpIfTrue(label) => self.op_jump_if_true(label),
                Opcode::JumpIfFalse(label) => self.op_jump_if_false(label),
                // Return operation
                Opcode::Return => return,
                // Call operation
                Opcode::Call(arity) => self.op_call(arity),
                // Field operations
                Opcode::LoadField(field) => self.op_load_field(field),
                Opcode::StoreField(field) => self.op_store_field(field),
                Opcode::DefineField(field) => self.op_define_field(field),
                // Load/store/define operations
                Opcode::Load(name) => self.op_load(name),
                Opcode::Store(name) => self.op_store(name),
                Opcode::Define(name) => self.op_define(name),
                // Make operations
                Opcode::MakeClosure(function) => self.op_make_closure(function),
                Opcode::MakeClass(name, methods) => self.op_make_class(name, methods),
                Opcode::MakeTrait(name, functions) => self.op_make_trait(name, functions),
                // Import operation
                Opcode::Import(path) => self.op_import(path),
            }

            // Switching to next instruction
            self.frame_mut().step();
        }
    }
}
