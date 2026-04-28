/// Imports
use crate::{
    errors::RuntimeError,
    interpreter::Interpreter,
    refs::{RealmRef, Ref},
    rt::{
        flow::{ControlFlow, Flow},
        realm::Realm,
        value::{Callable, Class, Closure, Enum, Function, Method, Trait, TraitFunction, Value},
    },
};
use geko_common::{bail, bug};
use geko_ir::{
    atom::{self, AssignOp, BinOp},
    expr::Expression,
    stmt::{Block, Statement, UseKind},
};
use geko_lex::token::Span;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Statement execution implementation
impl<'io> Interpreter<'io> {
    /// Executes while statement
    fn exec_while(&mut self, span: &Span, condition: &Expression, block: &Block) -> Flow<()> {
        // Evaluating condition value
        let mut value = self.eval(condition)?;

        // While conditions is true
        while self.is_truthy(span, &value) {
            // Executing body and re-evaluating condition value
            match self.exec_block(block, true) {
                Ok(_) => {
                    value = self.eval(condition)?;
                    continue;
                }
                Err(flow) => match flow {
                    ControlFlow::Break => break,
                    ControlFlow::Continue => continue,
                    other => return Err(other),
                },
            };
        }

        Ok(())
    }

    /// Executes for statement
    fn exec_for(
        &mut self,
        span: &Span,
        var: &str,
        iterable: &Expression,
        block: &Block,
    ) -> Flow<()> {
        // Invalid iterator error
        let invalid_iterator = |value: Value| {
            bail!(RuntimeError::InvalidIterator {
                src: span.0.clone(),
                span: span.1.clone().into(),
                value
            })
        };

        // Corrupted iterator error
        let corrupted_iterator = || {
            bail!(RuntimeError::CorruptedIterator {
                src: span.0.clone(),
                span: span.1.clone().into(),
            })
        };

        // Retrieving list class
        let list_class = {
            let list_value = self
                .builtins
                .env
                .borrow()
                .lookup("List")
                .unwrap_or_else(|| bug!("no builtin `List` found"));

            match list_value {
                Value::Class(t) => t,
                _ => bug!("builtin `List` is not a class"),
            }
        };

        // Retrieving iterator
        let vec = match self.eval(iterable)? {
            // Matching list
            Value::Instance(instance) => {
                // Checking classes equality
                if Rc::ptr_eq(&instance.borrow().type_of, &list_class) {
                    // Safety: borrow is temporal for this line
                    let internal = instance
                        .borrow_mut()
                        .fields
                        .get("$internal")
                        .cloned()
                        .unwrap();

                    match internal {
                        Value::Any(list) => match list.borrow_mut().downcast_mut::<Vec<Value>>() {
                            Some(vec) => vec.clone(),
                            _ => corrupted_iterator(),
                        },
                        _ => corrupted_iterator(),
                    }
                } else {
                    invalid_iterator(Value::Instance(instance))
                }
            }
            value => invalid_iterator(value),
        };

        // Iterating over vector
        for i in vec {
            // Preparing environment
            let mut env = Realm::new(self.realm.clone());
            env.define(var, i);

            match self.exec_block_with(block, RealmRef::new(RefCell::new(env))) {
                Ok(_) => {
                    continue;
                }
                Err(flow) => match flow {
                    ControlFlow::Break => break,
                    ControlFlow::Continue => continue,
                    other => return Err(other),
                },
            };
        }

        Ok(())
    }

    /// Executes if statement
    fn exec_if(
        &mut self,
        span: &Span,
        condition: &Expression,
        then: &Block,
        else_: &Option<Box<Statement>>,
    ) -> Flow<()> {
        // Evaluating condition value
        let value = self.eval(condition)?;

        // If condition is true
        if self.is_truthy(span, &value) {
            self.exec_block(then, true)?;
        }
        // Else
        else if let Some(else_) = else_ {
            self.exec(else_)?;
        }

        Ok(())
    }

    /// Executes class statement
    fn exec_class_decl(&mut self, class: &atom::Class) -> Flow<()> {
        // Creating class
        let class_ref = Ref::new(Class {
            name: class.name.to_string(),
            methods: class
                .methods
                .iter()
                .map(|method| {
                    (
                        method.name.clone(),
                        Method::Closure(Ref::new(Closure {
                            function: Ref::new(Function {
                                params: method.params.clone(),
                                block: method.block.clone(),
                            }),
                            realm: self.realm.clone(),
                        })),
                    )
                })
                .collect(),
        });

        // Defining class in the environment
        self.realm
            .borrow_mut()
            .define(&class.name, Value::Class(class_ref));

        Ok(())
    }

    /// Executes enum statement
    fn exec_enum_decl(&mut self, en: &atom::Enum) -> Flow<()> {
        // Creating enum
        let enum_ref = Ref::new(Enum {
            name: en.name.to_string(),
            variants: en.variants.to_owned(),
        });

        // Defining enum in the environment
        self.realm
            .borrow_mut()
            .define(&en.name, Value::Enum(enum_ref));

        Ok(())
    }

    /// Executes trait statement
    fn exec_trait_decl(&mut self, trt: &atom::Trait) -> Flow<()> {
        // Creating trait
        let trait_ref = Ref::new(Trait {
            name: trt.name.to_string(),
            functions: trt
                .functions
                .iter()
                .map(|f| TraitFunction {
                    name: f.name.clone(),
                    arity: f.params.len(),
                })
                .collect(),
        });

        // Defining enum in the environment
        self.realm
            .borrow_mut()
            .define(&trt.name, Value::Trait(trait_ref));

        Ok(())
    }

    /// Executes function statement
    fn exec_function_decl(&mut self, function: &atom::Function) -> Flow<()> {
        // Creating function
        let function_ref = Ref::new(Function {
            params: function.params.clone(),
            block: function.block.clone(),
        });

        // Capturing environment with function
        let closure = Ref::new(Closure {
            function: function_ref,
            realm: self.realm.clone(),
        });

        // Defining function in the environment
        self.realm
            .borrow_mut()
            .define(&function.name, Value::Callable(Callable::Closure(closure)));

        Ok(())
    }

    /// Executes assignment
    fn exec_assign(
        &mut self,
        span: &Span,
        name: &str,
        op: &AssignOp,
        value: &Expression,
    ) -> Flow<()> {
        // Evaluating value
        let value = self.eval(value)?;

        // Matching operator
        match op {
            // Processing field definition
            AssignOp::Define => self.realm.borrow_mut().define(name, value),
            // Processing field assignment
            AssignOp::Assign => self.realm.borrow_mut().set(span, name, value),
            // Otherwise, processing assignment with compound operation
            other => {
                // Old variable value
                let old = self.eval_variable(span, name)?;

                // Performing operation
                let value = self.perform_bin_op(
                    span,
                    old,
                    value,
                    match other {
                        // Note: because of previous clause
                        AssignOp::Assign | AssignOp::Define => unreachable!(),
                        AssignOp::Add => BinOp::Add,
                        AssignOp::Sub => BinOp::Sub,
                        AssignOp::Mul => BinOp::Mul,
                        AssignOp::Div => BinOp::Div,
                        AssignOp::Mod => BinOp::Mod,
                        AssignOp::BitAnd => BinOp::BitAnd,
                        AssignOp::BitOr => BinOp::BitOr,
                        AssignOp::Xor => BinOp::Xor,
                    },
                );

                // Processing assignment
                self.realm.borrow_mut().set(span, name, value);
            }
        }

        Ok(())
    }

    /// Defines field
    fn define_field(span: &Span, container: Value, name: &str, value: Value) -> Flow<()> {
        // Matching container
        match container {
            // Module field assignment
            Value::Module(m) => m.borrow_mut().env.borrow_mut().define(name, value),
            // Instance field assignment
            Value::Instance(i) => {
                i.borrow_mut().fields.insert(name.to_string(), value);
            }
            // Otherwise, raising error
            value => bail!(RuntimeError::CouldNotResolveFields {
                src: span.0.clone(),
                span: span.1.clone().into(),
                value
            }),
        }

        Ok(())
    }

    /// Assigns field
    fn assign_field(span: &Span, container: Value, name: &str, value: Value) -> Flow<()> {
        // Matching container
        match container {
            // Module field assignment
            Value::Module(m) => m.borrow_mut().env.borrow_mut().set(span, name, value),
            // Instance field assignment if field exists
            Value::Instance(i) if i.borrow().fields.contains_key(name) => {
                i.borrow_mut().fields.insert(name.to_string(), value);
            }
            // Instance field assignment if field doesn't exist
            Value::Instance(_) => {
                bail!(RuntimeError::UndefinedField {
                    name: name.to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                })
            }
            // Otherwise, raising error
            value => bail!(RuntimeError::CouldNotResolveFields {
                src: span.0.clone(),
                span: span.1.clone().into(),
                value
            }),
        }

        Ok(())
    }

    /// Executes field set
    fn exec_set(
        &mut self,
        span: &Span,
        container: &Expression,
        name: &str,
        op: &AssignOp,
        value: &Expression,
    ) -> Flow<()> {
        // Evaluating container
        let container = self.eval(container)?;
        // Evaluating value
        let value = self.eval(value)?;

        // Matching operator
        match op {
            // Processing field definition
            AssignOp::Define => Self::define_field(span, container, name, value),
            // Processing field assignment
            AssignOp::Assign => Self::assign_field(span, container, name, value),
            // Otherwise, processing assignment with compound operation
            other => {
                // Old field value
                let old = Self::access_field(span, name, container.clone())?;

                // Performing operation
                let value = self.perform_bin_op(
                    span,
                    old,
                    value,
                    match other {
                        // Note: because of previous clause
                        AssignOp::Assign | AssignOp::Define => unreachable!(),
                        AssignOp::Add => BinOp::Add,
                        AssignOp::Sub => BinOp::Sub,
                        AssignOp::Mul => BinOp::Mul,
                        AssignOp::Div => BinOp::Div,
                        AssignOp::Mod => BinOp::Mod,
                        AssignOp::BitAnd => BinOp::BitAnd,
                        AssignOp::BitOr => BinOp::BitOr,
                        AssignOp::Xor => BinOp::Xor,
                    },
                );

                // Processing assignment
                Self::assign_field(span, container, name, value)
            }
        }
    }

    /// Executes return
    fn exec_return(&mut self, expr: &Option<Expression>) -> Flow<()> {
        match expr {
            Some(expr) => {
                let value = self.eval(expr)?;
                Err(ControlFlow::Return(value))
            }
            None => Err(ControlFlow::Return(Value::Null)),
        }
    }

    /// Executes continue
    fn exec_continue(&mut self) -> Flow<()> {
        Err(ControlFlow::Continue)
    }

    /// Executes break
    fn exec_break(&mut self) -> Flow<()> {
        Err(ControlFlow::Break)
    }

    /// Executes use
    fn exec_use(&mut self, span: &Span, name: &str, kind: &UseKind) -> Flow<()> {
        // Resolving use path
        let module = {
            // Resolving fs path
            match self.io.resolve(name) {
                Some(path) => self.interpret_module(name, &self.io.read(&path)),
                None => match self.load_builtin_module(name) {
                    Some(module) => module,
                    None => bail!(RuntimeError::FailedToFindModule {
                        name: name.to_string(),
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    }),
                },
            }
        };

        // Checking usage kind
        match kind {
            UseKind::As(name) => self.realm.borrow_mut().define(name, Value::Module(module)),
            UseKind::For(items) => {
                // Crawling items
                let items: HashMap<String, Value> = {
                    let module = module.borrow();
                    let env = module.env.borrow();
                    items
                        .iter()
                        .map(|name| {
                            (
                                name.clone(),
                                match env.lookup(name) {
                                    Some(value) => value,
                                    None => bail!(RuntimeError::UndefinedField {
                                        name: name.clone(),
                                        src: span.0.clone(),
                                        span: span.1.clone().into()
                                    }),
                                },
                            )
                        })
                        .collect()
                };
                // Defining items
                items
                    .into_iter()
                    .for_each(|(k, v)| self.realm.borrow_mut().define(&k, v));
            }
            UseKind::All => {
                // Crawling items
                let items: HashMap<String, Value> = {
                    let module = module.borrow();
                    let env = module.env.borrow();
                    env.variables.clone()
                };
                // Defining items
                items
                    .into_iter()
                    .for_each(|(k, v)| self.realm.borrow_mut().define(&k, v));
            }
            UseKind::Just => self
                .realm
                .borrow_mut()
                // Safety: `split()` returns iterator with at least 1 element
                .define(name.split("/").last().unwrap(), Value::Module(module)),
        }

        Ok(())
    }

    /// Executes statement
    pub fn exec(&mut self, stmt: &Statement) -> Flow<()> {
        // Matching statement
        match stmt {
            Statement::While {
                span,
                condition,
                block,
            } => self.exec_while(span, condition, block),
            Statement::If {
                span,
                condition,
                then,
                else_,
            } => self.exec_if(span, condition, then, else_),
            Statement::Class(class) => self.exec_class_decl(class),
            Statement::Enum(en) => self.exec_enum_decl(en),
            Statement::Trait(trt) => self.exec_trait_decl(trt),
            Statement::Function(function) => self.exec_function_decl(function),
            Statement::Assign {
                span,
                name,
                op,
                value,
            } => self.exec_assign(span, name, op, value),
            Statement::Set {
                span,
                container,
                name,
                op,
                value,
            } => self.exec_set(span, container, name, op, value),
            Statement::Return { expr, .. } => self.exec_return(expr),
            Statement::Continue(_) => self.exec_continue(),
            Statement::Break(_) => self.exec_break(),
            Statement::Expr(expression) => {
                self.eval(expression)?;
                Ok(())
            }
            Statement::Block(block) => self.exec_block(block, true),
            Statement::Use { span, path, kind } => self.exec_use(span, path, kind),
            Statement::For {
                span,
                var,
                iterable,
                block,
            } => self.exec_for(span, var, iterable, block),
        }
    }

    /// Executes block
    pub fn exec_block(&mut self, block: &Block, new_scope: bool) -> Flow<()> {
        // If block requires new scope
        if new_scope {
            // Pushing scope
            let previous = self.realm.clone();
            self.realm = RealmRef::new(RefCell::new(Realm::new(previous.clone())));

            // Executing statements
            for stmt in &block.statements {
                self.exec(stmt)?;
            }

            // Popping scope
            self.realm = previous;
        }
        // If not
        else {
            // Executing statements
            for stmt in &block.statements {
                self.exec(stmt)?;
            }
        }

        Ok(())
    }

    /// Executes block with environment
    pub fn exec_block_with(&mut self, block: &Block, env: RealmRef) -> Flow<()> {
        // Pushing scope
        let previous = self.realm.clone();
        self.realm = env;

        // Executing statements
        for stmt in &block.statements {
            self.exec(stmt)?;
        }

        // Popping scope
        self.realm = previous;
        Ok(())
    }
}
