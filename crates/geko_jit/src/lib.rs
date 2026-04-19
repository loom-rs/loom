/// Modules
pub mod meta;

/// Imports
use crate::meta::{Signature, Typ, Variable};
use cranelift::{
    codegen,
    prelude::{
        self, AbiParam, Configurable, FloatCC, FunctionBuilder, FunctionBuilderContext,
        InstBuilder, IntCC, settings, types,
    },
};
use cranelift_codegen::ir::FuncRef;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use geko_ir::{
    atom::{AssignOp, BinOp, Lit, UnaryOp},
    expr::Expression,
    stmt::{self, Block, Statement},
};
use std::collections::HashMap;

/// Code generation error
#[derive(Debug)]
pub enum Error {
    /// Could not generate code for function
    NoJitEligible,

    /// No jit code generated yet
    NoJitCode,

    /// Host machine is not supported
    HostMachineNotSupported,

    /// Code generation failure
    GenerationFailure,

    /// Function declartion in the module failure
    ModuleDeclarationFailure,

    /// Function definition in the module failure
    ModuleDefinitionFailure,
}

/// Represents cranelift function context
pub struct FunctionContext<'ctx> {
    /// Function builder
    builder: FunctionBuilder<'ctx>,

    /// Variables map
    variables: HashMap<String, Variable>,

    /// Loops stack (continue block, break block)
    loops: Vec<(prelude::Block, prelude::Block)>,

    /// Function signature
    signature: Signature,

    /// Recursion function reference
    rec_ref: FuncRef,
}

/// Function context implementation
impl<'ctx> FunctionContext<'ctx> {
    /// Declares variable
    fn declare_var(&mut self, name: &str, typ: &Typ) -> Variable {
        *self
            .variables
            .entry(name.into())
            .or_insert_with(|| Variable {
                variable: self.builder.declare_var(CodeGenerator::map_type(typ)),
                typ: typ.clone(),
            })
    }

    /// Looks up variable
    fn lookup_var(&self, name: &str) -> Option<Variable> {
        self.variables.get(name).cloned()
    }

    /// Returns true if block has terminator
    fn has_terminator(&self) -> bool {
        // Matching current block
        match self.builder.current_block() {
            // Matching last block instruction
            Some(block) => match self.builder.func.layout.last_inst(block) {
                // Empty block contains no terminator
                None => false,
                // Non-empty block may contain terminator
                Some(inst) => {
                    // Check if the last instruction is a terminator
                    self.builder.func.dfg.insts[inst].opcode().is_terminator()
                }
            },
            // If no block
            None => false,
        }
    }

    /// Translates AST into Cranelift IR
    fn translate(&mut self, block: &Block) -> Result<(), Error> {
        for stmt in &block.statements {
            self.translate_stmt(stmt)?;
        }

        Ok(())
    }

    /// Translates assignment into Cranelift IR
    fn translate_assign(
        &mut self,
        name: &str,
        op: AssignOp,
        value: &Expression,
    ) -> Result<(), Error> {
        let var = self.lookup_var(name).ok_or(Error::NoJitEligible)?;
        let (typ, val) = self.translate_expr(value)?;

        // Types missmatch check
        if typ != var.typ {
            return Err(Error::NoJitEligible);
        }

        // Matching on op with different codegen result
        match op {
            // Default definition operator
            AssignOp::Define => {
                let (typ, val) = self.translate_expr(value)?;

                let var = self.declare_var(name, &typ);
                self.builder.def_var(var.variable, val);
            }
            // Default assignment operator
            AssignOp::Assign => {
                self.builder.def_var(var.variable, val);
            }
            // Compound operators
            AssignOp::Add => {
                let prev = self.builder.use_var(var.variable);
                let val = match var.typ {
                    Typ::Int => self.builder.ins().iadd(prev, val),
                    Typ::Float => self.builder.ins().fadd(prev, val),
                    _ => return Err(Error::NoJitEligible),
                };
                self.builder.def_var(var.variable, val);
            }
            AssignOp::Sub => {
                let prev = self.builder.use_var(var.variable);
                let val = match var.typ {
                    Typ::Int => self.builder.ins().isub(prev, val),
                    Typ::Float => self.builder.ins().fsub(prev, val),
                    _ => return Err(Error::NoJitEligible),
                };
                self.builder.def_var(var.variable, val);
            }
            AssignOp::Mul => {
                let prev = self.builder.use_var(var.variable);
                let val = match var.typ {
                    Typ::Int => self.builder.ins().imul(prev, val),
                    Typ::Float => self.builder.ins().fmul(prev, val),
                    _ => return Err(Error::NoJitEligible),
                };
                self.builder.def_var(var.variable, val);
            }
            AssignOp::Div => {
                let prev = self.builder.use_var(var.variable);
                let val = match var.typ {
                    Typ::Int => self.builder.ins().sdiv(prev, val),
                    Typ::Float => self.builder.ins().fdiv(prev, val),
                    _ => return Err(Error::NoJitEligible),
                };
                self.builder.def_var(var.variable, val);
            }
            AssignOp::Mod => {
                let prev = self.builder.use_var(var.variable);
                let val = match var.typ {
                    Typ::Int => self.builder.ins().srem(prev, val),
                    _ => return Err(Error::NoJitEligible),
                };
                self.builder.def_var(var.variable, val);
            }
            AssignOp::BitAnd => {
                let prev = self.builder.use_var(var.variable);
                let val = match var.typ {
                    Typ::Int => self.builder.ins().band(prev, val),
                    Typ::Bool => self.builder.ins().band(prev, val),
                    _ => return Err(Error::NoJitEligible),
                };
                self.builder.def_var(var.variable, val);
            }
            AssignOp::BitOr => {
                let prev = self.builder.use_var(var.variable);
                let val = match var.typ {
                    Typ::Int => self.builder.ins().bor(prev, val),
                    Typ::Bool => self.builder.ins().bor(prev, val),
                    _ => return Err(Error::NoJitEligible),
                };
                self.builder.def_var(var.variable, val);
            }
            AssignOp::Xor => {
                let prev = self.builder.use_var(var.variable);
                let val = match var.typ {
                    Typ::Int => self.builder.ins().bxor(prev, val),
                    Typ::Bool => self.builder.ins().bxor(prev, val),
                    _ => return Err(Error::NoJitEligible),
                };
                self.builder.def_var(var.variable, val);
            }
        }

        Ok(())
    }

    // Translates return into Cranelift IR
    fn translate_return(&mut self, expr: &Option<Expression>) -> Result<(), Error> {
        // If current block already has terminator
        if self.has_terminator() {
            return Err(Error::NoJitEligible);
        }

        // Matching expression and return type
        match (expr, self.signature.ret) {
            (Some(expr), Some(ret_typ)) => {
                let (typ, val) = self.translate_expr(expr)?;

                // Types missmatch check
                if typ != ret_typ {
                    return Err(Error::NoJitEligible);
                }

                self.builder.ins().return_(&[val]);
                Ok(())
            }
            (None, None) => {
                self.builder.ins().return_(&[]);
                Ok(())
            }
            (_, _) => Err(Error::NoJitEligible),
        }
    }

    // Translates continue into Cranelift IR
    fn translate_continue(&mut self) -> Result<(), Error> {
        // If current block already has terminator
        if self.has_terminator() {
            return Err(Error::NoJitEligible);
        }

        let (continue_block, _) = self.loops.last().ok_or(Error::NoJitEligible)?;
        self.builder.ins().jump(*continue_block, &[]);
        Ok(())
    }

    // Translates break into Cranelift IR
    fn translate_break(&mut self) -> Result<(), Error> {
        // If current block already has terminator
        if self.has_terminator() {
            return Err(Error::NoJitEligible);
        }

        let (_, break_block) = self.loops.last().ok_or(Error::NoJitEligible)?;
        self.builder.ins().jump(*break_block, &[]);
        Ok(())
    }

    // Translates block into Cranelift IR
    fn translate_block(&mut self, block: &Block) -> Result<(), Error> {
        for stmt in &block.statements {
            self.translate_stmt(&stmt)?;
        }
        Ok(())
    }

    // Translates while into Cranelift IR
    fn translate_while(&mut self, condition: &Expression, block: &Block) -> Result<(), Error> {
        // Block definitions
        let header = self.builder.create_block();
        let body = self.builder.create_block();
        let exit = self.builder.create_block();

        // Jumping into header
        self.builder.ins().jump(header, &[]);

        // Jumps to body if condition is true, else jumps to exit block
        self.builder.switch_to_block(header);
        let (_, cond_val) = self.translate_expr(condition)?;
        self.builder.ins().brif(cond_val, body, &[], exit, &[]);

        // Body block
        self.builder.switch_to_block(body);
        self.loops.push((header, exit));
        self.translate_block(block)?;
        self.loops.pop();
        self.builder.ins().jump(header, &[]);

        // Switching to exit block
        self.builder.switch_to_block(exit);

        Ok(())
    }

    // Translates if into Cranelift IR
    fn translate_if(
        &mut self,
        condition: &Expression,
        then: &Block,
        else_: &Option<Box<Statement>>,
    ) -> Result<(), Error> {
        // Header block is an entry point of `if`
        let header = self.builder.create_block();

        // Jumping into header
        self.builder.ins().jump(header, &[]);

        // Exit block
        let exit = self.builder.create_block();

        // Then block
        let then = {
            let block = self.builder.create_block();
            self.builder.switch_to_block(block);

            self.translate_block(then)?;
            if !self.has_terminator() {
                self.builder.ins().jump(exit, &[]);
            }

            block
        };

        // Else block
        let else_ = {
            let block = self.builder.create_block();
            self.builder.switch_to_block(block);

            match else_ {
                Some(else_) => {
                    self.translate_stmt(else_)?;
                }
                None => {
                    if !self.has_terminator() {
                        self.builder.ins().jump(exit, &[]);
                    }
                }
            }

            block
        };

        // If condition is true jumping to body, else to else_ block
        self.builder.switch_to_block(header);
        let (_, cond_val) = self.translate_expr(condition)?;
        self.builder.ins().brif(cond_val, then, &[], else_, &[]);

        // Switching to exit block
        self.builder.switch_to_block(exit);

        Ok(())
    }

    /// Translates statement into Cranelift IR
    fn translate_stmt(&mut self, stmt: &Statement) -> Result<(), Error> {
        match stmt {
            Statement::While {
                condition, block, ..
            } => self.translate_while(condition, block),
            Statement::If {
                condition,
                then,
                else_,
                ..
            } => self.translate_if(condition, then, else_),
            Statement::Assign {
                name, op, value, ..
            } => self.translate_assign(name, *op, value),
            Statement::Return { expr, .. } => self.translate_return(expr),
            Statement::Continue(_) => self.translate_continue(),
            Statement::Break(_) => self.translate_break(),
            Statement::Expr(expr) => {
                self.translate_expr(expr)?;
                Ok(())
            }
            Statement::Block(block) => {
                self.translate_block(block)?;
                Ok(())
            }
            Statement::For { .. }
            | Statement::Class { .. }
            | Statement::Enum { .. }
            | Statement::Trait { .. }
            | Statement::Use { .. }
            | Statement::Function(_)
            | Statement::Bail { .. }
            | Statement::Set { .. } => Err(Error::NoJitEligible),
        }
    }

    /// Translates literal into Cranelift IR
    fn translate_lit(&mut self, lit: &Lit) -> Result<(Typ, prelude::Value), Error> {
        match lit {
            Lit::Number(num) => {
                if num.contains(".") {
                    let lit: f64 = num.parse().unwrap();
                    Ok((Typ::Float, self.builder.ins().f64const(lit)))
                } else {
                    let lit: i64 = num.parse().unwrap();
                    Ok((Typ::Int, self.builder.ins().iconst(types::I64, lit)))
                }
            }
            Lit::Bool(lit) => {
                let lit: i64 = if lit == "true" { 1 } else { 0 };
                Ok((Typ::Bool, self.builder.ins().iconst(types::I8, lit)))
            }
            Lit::Null | Lit::String(_) => Err(Error::NoJitEligible),
        }
    }

    /// Translates binary operation into Cranelift IR
    fn translate_binary(
        &mut self,
        op: BinOp,
        lhs: &Expression,
        rhs: &Expression,
    ) -> Result<(Typ, prelude::Value), Error> {
        // Translating lhs and rhs
        let (lhs_typ, lhs_val) = self.translate_expr(lhs)?;
        let (rhs_typ, rhs_val) = self.translate_expr(rhs)?;

        // Calculating operands type and values
        let (l_val, r_val, typ) = match (lhs_typ, rhs_typ) {
            (Typ::Float, _) => (lhs_val, self.to_float(rhs_val, rhs_typ)?, Typ::Float),
            (_, Typ::Float) => (self.to_float(lhs_val, lhs_typ)?, rhs_val, Typ::Float),
            _ => (lhs_val, rhs_val, lhs_typ),
        };

        // Calculating result value and type
        let (typ, val) = match op {
            // Arithmetical operations
            BinOp::Add => match typ {
                Typ::Int => (Typ::Int, self.builder.ins().iadd(l_val, r_val)),
                Typ::Float => (Typ::Float, self.builder.ins().fadd(l_val, r_val)),
                _ => return Err(Error::NoJitEligible),
            },
            BinOp::Sub => match typ {
                Typ::Int => (Typ::Int, self.builder.ins().isub(l_val, r_val)),
                Typ::Float => (Typ::Float, self.builder.ins().fsub(l_val, r_val)),
                _ => return Err(Error::NoJitEligible),
            },
            BinOp::Mul => match typ {
                Typ::Int => (Typ::Int, self.builder.ins().imul(l_val, r_val)),
                Typ::Float => (Typ::Float, self.builder.ins().fmul(l_val, r_val)),
                _ => return Err(Error::NoJitEligible),
            },
            BinOp::Div => match typ {
                Typ::Int => (Typ::Int, self.builder.ins().sdiv(l_val, r_val)),
                Typ::Float => (Typ::Float, self.builder.ins().fdiv(l_val, r_val)),
                _ => return Err(Error::NoJitEligible),
            },
            // Note: float reminder operation not supported
            BinOp::Mod => match typ {
                Typ::Int => (Typ::Int, self.builder.ins().srem(l_val, r_val)),
                _ => return Err(Error::NoJitEligible),
            },
            // Compare operations
            BinOp::Eq => (
                Typ::Bool,
                match typ {
                    Typ::Int => self.builder.ins().icmp(IntCC::Equal, l_val, r_val),
                    Typ::Bool => self.builder.ins().icmp(IntCC::Equal, l_val, r_val),
                    Typ::Float => self.builder.ins().fcmp(FloatCC::Equal, l_val, r_val),
                },
            ),
            BinOp::Ne => (
                Typ::Bool,
                match typ {
                    Typ::Int => self.builder.ins().icmp(IntCC::NotEqual, l_val, r_val),
                    Typ::Bool => self.builder.ins().icmp(IntCC::NotEqual, l_val, r_val),
                    Typ::Float => self.builder.ins().fcmp(FloatCC::NotEqual, l_val, r_val),
                },
            ),
            BinOp::Lt => (
                Typ::Bool,
                match typ {
                    Typ::Int => self.builder.ins().icmp(IntCC::SignedLessThan, l_val, r_val),
                    Typ::Bool => self.builder.ins().icmp(IntCC::SignedLessThan, l_val, r_val),
                    Typ::Float => self.builder.ins().fcmp(FloatCC::LessThan, l_val, r_val),
                },
            ),
            BinOp::Gt => (
                Typ::Bool,
                match typ {
                    Typ::Int => self
                        .builder
                        .ins()
                        .icmp(IntCC::SignedGreaterThan, l_val, r_val),
                    Typ::Bool => self
                        .builder
                        .ins()
                        .icmp(IntCC::SignedGreaterThan, l_val, r_val),
                    Typ::Float => self.builder.ins().fcmp(FloatCC::GreaterThan, l_val, r_val),
                },
            ),
            BinOp::Le => (
                Typ::Bool,
                match typ {
                    Typ::Int => self
                        .builder
                        .ins()
                        .icmp(IntCC::SignedLessThanOrEqual, l_val, r_val),
                    Typ::Bool => {
                        self.builder
                            .ins()
                            .icmp(IntCC::SignedLessThanOrEqual, l_val, r_val)
                    }
                    Typ::Float => self
                        .builder
                        .ins()
                        .fcmp(FloatCC::LessThanOrEqual, l_val, r_val),
                },
            ),
            BinOp::Ge => (
                Typ::Bool,
                match typ {
                    Typ::Int => {
                        self.builder
                            .ins()
                            .icmp(IntCC::SignedGreaterThanOrEqual, l_val, r_val)
                    }
                    Typ::Bool => {
                        self.builder
                            .ins()
                            .icmp(IntCC::SignedGreaterThanOrEqual, l_val, r_val)
                    }
                    Typ::Float => {
                        self.builder
                            .ins()
                            .fcmp(FloatCC::GreaterThanOrEqual, l_val, r_val)
                    }
                },
            ),

            // Logical operations
            BinOp::And => match typ {
                Typ::Bool => (Typ::Bool, self.builder.ins().band(l_val, r_val)),
                _ => return Err(Error::NoJitEligible),
            },
            BinOp::Or => match typ {
                Typ::Bool => (Typ::Bool, self.builder.ins().bor(l_val, r_val)),
                _ => return Err(Error::NoJitEligible),
            },
            BinOp::Xor => match typ {
                Typ::Bool => (Typ::Bool, self.builder.ins().bxor(l_val, r_val)),
                Typ::Int => (Typ::Int, self.builder.ins().bxor(l_val, r_val)),
                _ => return Err(Error::NoJitEligible),
            },
            BinOp::BitAnd => match typ {
                Typ::Bool => (Typ::Bool, self.builder.ins().band(l_val, r_val)),
                Typ::Int => (Typ::Int, self.builder.ins().band(l_val, r_val)),
                _ => return Err(Error::NoJitEligible),
            },
            BinOp::BitOr => match typ {
                Typ::Bool => (Typ::Bool, self.builder.ins().bor(l_val, r_val)),
                Typ::Int => (Typ::Int, self.builder.ins().bor(l_val, r_val)),
                _ => return Err(Error::NoJitEligible),
            },
            BinOp::Impls | BinOp::NotImpls => return Err(Error::NoJitEligible),
        };

        Ok((typ, val))
    }

    /// Translates unary operation into Cranelift IR
    fn translate_unary(
        &mut self,
        op: UnaryOp,
        val: &Expression,
    ) -> Result<(Typ, prelude::Value), Error> {
        // Translating value
        let (typ, val) = self.translate_expr(val)?;

        // Calculating result value
        let val = match op {
            UnaryOp::Neg => match typ {
                Typ::Int => self.builder.ins().ineg(val),
                Typ::Float => self.builder.ins().fneg(val),
                _ => return Err(Error::NoJitEligible),
            },
            UnaryOp::Bang => match typ {
                Typ::Bool => {
                    let one = self.builder.ins().iconst(types::I8, 1);
                    self.builder.ins().bxor(val, one)
                }
                _ => return Err(Error::NoJitEligible),
            },
        };

        Ok((typ, val))
    }

    /// Tries transalate call if it is a recursion call into Cranelift IR
    fn try_translate_call(
        &mut self,
        what: &Expression,
        args: &[Expression],
    ) -> Result<(Typ, prelude::Value), Error> {
        match what {
            // Todo: replace with `if let` guard in `rust 1.95`
            Expression::Variable { name, .. }
                if self.lookup_var(name).is_none() && &self.signature.name == name.as_str() =>
            {
                // Preparing function arguments
                let args: Vec<_> = args
                    .iter()
                    .map(|e| self.translate_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;

                // Checking signature
                self.signature
                    .params
                    .iter()
                    .zip(args.iter())
                    .try_for_each(|(p, a)| {
                        if p.typ == a.0 {
                            Ok(())
                        } else {
                            Err(Error::NoJitEligible)
                        }
                    })?;

                // Generating function call
                let inst = self.builder.ins().call(
                    self.rec_ref,
                    &args.iter().map(|(_, v)| *v).collect::<Vec<_>>(),
                );
                let result = self.builder.inst_results(inst);

                match self.signature.ret {
                    Some(typ) => Ok((typ, result[0])),
                    None => Err(Error::NoJitEligible),
                }
            }
            // Unsupported call
            _ => Err(Error::NoJitEligible),
        }
    }

    /// Translates expression into Cranelift IR
    fn translate_expr(&mut self, expr: &Expression) -> Result<(Typ, prelude::Value), Error> {
        match expr {
            // Literal translation
            Expression::Lit { lit, .. } => self.translate_lit(lit),
            // Binary op translation
            Expression::Bin { op, lhs, rhs, .. } => self.translate_binary(*op, lhs, rhs),
            Expression::Unary { op, value, .. } => self.translate_unary(*op, value),
            Expression::Variable { name, .. } => match self.lookup_var(name) {
                Some(var) => Ok((var.typ, self.builder.use_var(var.variable))),
                None => Err(Error::NoJitEligible),
            },
            // Recursing call
            Expression::Call { what, args, .. } => self.try_translate_call(what, args),
            // Unsupported operations
            Expression::Field { .. }
            | Expression::List { .. }
            | Expression::Dict { .. }
            | Expression::Fun { .. }
            | Expression::Range { .. } => Err(Error::NoJitEligible),
        }
    }

    /// Converts value to float
    fn to_float(&mut self, val: prelude::Value, typ: Typ) -> Result<prelude::Value, Error> {
        match typ {
            Typ::Int => Ok(self.builder.ins().fcvt_from_sint(types::F64, val)),
            Typ::Float => Ok(val),
            _ => Err(Error::NoJitEligible),
        }
    }
}

/// Represents cranelift code generator
pub struct CodeGenerator {
    /// Cranelift code generation context
    context: codegen::Context,

    /// Function build context
    builder_context: FunctionBuilderContext,

    /// Jit module
    jit_module: JITModule,
}

/// Implementation of the code generator
impl CodeGenerator {
    /// Creates new code generator
    pub fn new() -> Result<Self, Error> {
        // Building flags
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();

        // Building isa
        let isa_builder =
            cranelift_native::builder().map_err(|_| Error::HostMachineNotSupported)?;
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        // Building module
        let jit_module = JITModule::new(builder);

        Ok(Self {
            context: codegen::Context::new(),
            builder_context: FunctionBuilderContext::new(),
            jit_module,
        })
    }

    /// Performs code generation
    pub fn codegen(&mut self, sig: Signature, body: &stmt::Block) -> Result<*const u8, Error> {
        // Preparing function params
        sig.params.iter().for_each(|param| {
            self.context
                .func
                .signature
                .params
                .push(AbiParam::new(Self::map_type(&param.typ)))
        });

        // Preparing function return type if presented
        if let Some(ret) = sig.ret {
            self.context
                .func
                .signature
                .returns
                .push(AbiParam::new(Self::map_type(&ret)));
        }

        // Declaring function in the module
        let id = self
            .jit_module
            .declare_function(&sig.name, Linkage::Export, &self.context.func.signature)
            .map_err(|_| Error::ModuleDeclarationFailure)?;

        // Declaring function in function for recursion calls
        let rec_ref = self
            .jit_module
            .declare_func_in_func(id, &mut self.context.func);

        // Preparing entry block and function builder
        let mut builder = FunctionBuilder::new(&mut self.context.func, &mut self.builder_context);
        let entry_block = builder.create_block();

        // Preparing function context
        let mut context = FunctionContext {
            builder,
            variables: HashMap::new(),
            loops: Vec::new(),
            signature: sig.clone(),
            rec_ref: rec_ref,
        };

        // Preparing entry block
        context
            .builder
            .append_block_params_for_function_params(entry_block);
        context.builder.switch_to_block(entry_block);
        context.builder.seal_block(entry_block);

        // Preparing variables
        let args = context.builder.block_params(entry_block).to_vec();
        for (param, arg) in sig.params.iter().zip(args) {
            let var = context.declare_var(&param.name, &param.typ);
            context.builder.def_var(var.variable, arg);
        }

        // Translating ast into cranelift ir
        context.translate(body)?;

        // Sealing all blocks
        context.builder.seal_all_blocks();

        // Finalizing building function
        context.builder.finalize();

        // Defining function in the module
        self.jit_module
            .define_function(id, &mut self.context)
            .map_err(|_| Error::ModuleDefinitionFailure)?;

        // Printing out result
        println!("{}", self.context.func.display());

        // Compilation is done: clearing out context
        self.jit_module.clear_context(&mut self.context);

        // Finalizing mpodule definitions
        self.jit_module.finalize_definitions().unwrap();
        let code = self.jit_module.get_finalized_function(id);

        Ok(code)
    }

    /// Maps type
    fn map_type(typ: &Typ) -> types::Type {
        match typ {
            Typ::Int => types::I64,
            Typ::Float => types::F64,
            Typ::Bool => types::I8,
        }
    }
}
