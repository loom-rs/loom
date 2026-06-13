/// Imports
use crate::ast::{AssignOp, BinOp, Block, Expr, Lit, Stmt, UnOp};
use common::{bug, span::Span};
use dune::{
    ops::{Chunk, Label, Opcode},
    refs::Ref,
    value::Value,
};

/// Defines loop labels information
pub struct LoopLabels {
    /// Start label
    start_label: Label,

    /// End label
    end_label: Label,
}

/// Defines bytecode generator
#[derive(Default)]
pub struct CodeGenerator {
    /// Chunks stack
    chunks: Vec<Chunk>,

    /// Loops labels stack
    loops: Vec<LoopLabels>,
}

/// Implementation
impl CodeGenerator {
    /// Pushes new chunk onto the stack
    pub fn push_chunk(&mut self) {
        self.chunks.push(Chunk::default());
    }

    /// Pops chunk from the stack
    pub fn pop_chunk(&mut self) -> Chunk {
        self.chunks
            .pop()
            .unwrap_or_else(|| bug!("pop with empty chunk stack"))
    }

    /// Returns ref to last chunk in stack
    pub fn chunk(&mut self) -> &mut Chunk {
        self.chunks
            .last_mut()
            .unwrap_or_else(|| bug!("empty chunk stack"))
    }

    /// Pushes new loop info onto the stack
    pub fn push_loop(&mut self, labels: LoopLabels) {
        self.loops.push(labels);
    }

    /// Pops loops labels from the stack
    pub fn pop_loop(&mut self) -> LoopLabels {
        self.loops
            .pop()
            .unwrap_or_else(|| bug!("pop with empty loops stack"))
    }

    /// Performs generation of program
    pub fn gen_program(&mut self, program: Block) -> Ref<Chunk> {
        self.push_chunk();
        self.gen_block(program);
        Ref::new(self.pop_chunk())
    }

    /// Performs generation of block
    fn gen_block(&mut self, block: Block) {
        for stmt in block.stmts {
            self.gen_stmt(stmt);
        }
    }

    /// Performs generation of literal
    fn gen_lit(&mut self, span: Span, lit: Lit) {
        self.chunk().insert(
            span,
            Opcode::Push(match lit {
                Lit::Number(num) => {
                    if num.contains(".") {
                        Value::Float(num.parse().unwrap())
                    } else {
                        Value::Int(num.parse().unwrap())
                    }
                }
                Lit::String(str) => Value::String(str),
                Lit::Bool(bool) => Value::Bool(bool.parse().unwrap()),
                Lit::Null => Value::Null,
            }),
        );
    }

    /// Performs generation of binary operation
    fn gen_bin(&mut self, span: Span, op: BinOp, lhs: Expr, rhs: Expr) {
        self.gen_expr(lhs);
        self.gen_expr(rhs);

        self.chunk().insert(
            span,
            match op {
                BinOp::Add => Opcode::Add,
                BinOp::Sub => Opcode::Sub,
                BinOp::Mul => Opcode::Mul,
                BinOp::Div => Opcode::Div,
                BinOp::Mod => Opcode::Rem,
                BinOp::And => Opcode::And,
                BinOp::Or => Opcode::Or,
                BinOp::Gt => Opcode::Gt,
                BinOp::Ge => Opcode::Ge,
                BinOp::Lt => Opcode::Lt,
                BinOp::Le => Opcode::Le,
                BinOp::Eq => Opcode::Eq,
                BinOp::Ne => Opcode::Ne,
                BinOp::BitAnd => Opcode::Band,
                BinOp::BitOr => Opcode::Bor,
                BinOp::Xor => Opcode::Xor,
                BinOp::Impls => Opcode::Impls,
                BinOp::NotImpls => Opcode::NotImpls,
            },
        );
    }

    /// Performs generation of unary operation
    fn gen_un(&mut self, span: Span, op: UnOp, value: Expr) {
        self.gen_expr(value);
        self.chunk().insert(
            span,
            match op {
                UnOp::Neg => Opcode::Neg,
                UnOp::Bang => Opcode::Bang,
            },
        );
    }

    /// Performs generation of variable access
    fn gen_variable(&mut self, span: Span, name: String) {
        self.chunk().insert(span, Opcode::Load(name));
    }

    /// Performs generation of field access
    fn gen_field(&mut self, span: Span, name: String, container: Expr) {
        self.gen_expr(container);
        self.chunk().insert(span, Opcode::LoadField(name));
    }

    /// Performs generation of call
    fn gen_call(&mut self, span: Span, callee: Expr, args: Vec<Expr>) {
        self.gen_expr(callee);

        let arity = args.len();
        for arg in args {
            self.gen_expr(arg);
        }

        self.chunk().insert(span, Opcode::Call(arity));
    }

    /// Performs generation of expression
    fn gen_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Lit { span, lit } => self.gen_lit(span, lit),
            Expr::Bin { span, op, lhs, rhs } => self.gen_bin(span, op, *lhs, *rhs),
            Expr::Un { span, op, value } => self.gen_un(span, op, *value),
            Expr::Variable { span, name } => self.gen_variable(span, name),
            Expr::Field {
                span,
                name,
                container,
            } => self.gen_field(span, name, *container),
            Expr::Call { span, callee, args } => self.gen_call(span, *callee, args),
            Expr::List { span, list } => todo!(),
            Expr::Dict { span, dict } => todo!(),
            Expr::Fun {
                span,
                params,
                block,
            } => todo!(),
            Expr::Range {
                span,
                lhs,
                rhs,
                includes_end,
            } => todo!(),
        }
    }

    /// Performs generation of while loop
    pub fn gen_while(&mut self, span: Span, condition: Expr, block: Block) {
        // Preparing labels
        self.chunk().insert(span.clone(), Opcode::Nop);
        let start_label = self.chunk().fresh_label();
        let end_label = self.chunk().fresh_label();

        // Jumping to end if condition is false
        self.gen_expr(condition);
        self.chunk()
            .insert(span.clone(), Opcode::JumpIfFalse(end_label));

        // Loop body
        self.push_loop(LoopLabels {
            start_label,
            end_label,
        });
        self.gen_block(block);
        self.chunk().insert(span.clone(), Opcode::Jump(start_label));

        // Patching end label
        let end_pc = self.chunk().insert(span.clone(), Opcode::Nop);
        self.chunk().patch_label(end_label, end_pc);
    }

    /// Performs generation of if
    pub fn gen_if(&mut self, span: Span, condition: Expr, then: Block, else_: Option<Box<Stmt>>) {
        // Preparing labels
        self.chunk().insert(span.clone(), Opcode::Nop);
        let else_label = self.chunk().fresh_label();
        let end_label = self.chunk().fresh_label();

        // Jumping to else if condition is false
        self.gen_expr(condition);
        self.chunk()
            .insert(span.clone(), Opcode::JumpIfFalse(else_label));

        // Then block
        self.gen_block(then);
        self.chunk().insert(span.clone(), Opcode::Jump(end_label));

        // Else statement
        let else_pc = self.chunk().insert(span.clone(), Opcode::Nop);
        if let Some(stmt) = else_ {
            self.gen_stmt(*stmt);
        }

        // Patching lables
        let end_pc = self.chunk().insert(span, Opcode::Nop);
        self.chunk().patch_label(else_label, else_pc);
        self.chunk().patch_label(end_label, end_pc);
    }

    /// Performs generation of variable assign
    pub fn gen_variable_assign(&mut self, span: Span, name: String, op: AssignOp, value: Expr) {
        match op {
            AssignOp::Define => {
                self.gen_expr(value);
                self.chunk().insert(span, Opcode::Define(name));
            }
            AssignOp::Assign => {
                self.gen_expr(value);
                self.chunk().insert(span, Opcode::Store(name));
            }
            AssignOp::Add => {
                self.chunk()
                    .insert(span.clone(), Opcode::Load(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Add);
                self.chunk().insert(span, Opcode::Store(name));
            }
            AssignOp::Sub => {
                self.chunk()
                    .insert(span.clone(), Opcode::Load(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Sub);
                self.chunk().insert(span, Opcode::Store(name));
            }
            AssignOp::Mul => {
                self.chunk()
                    .insert(span.clone(), Opcode::Load(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Mul);
                self.chunk().insert(span, Opcode::Store(name));
            }
            AssignOp::Div => {
                self.chunk()
                    .insert(span.clone(), Opcode::Load(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Div);
                self.chunk().insert(span, Opcode::Store(name));
            }
            AssignOp::Mod => {
                self.chunk()
                    .insert(span.clone(), Opcode::Load(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Rem);
                self.chunk().insert(span, Opcode::Store(name));
            }
            AssignOp::BitAnd => {
                self.chunk()
                    .insert(span.clone(), Opcode::Load(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Band);
                self.chunk().insert(span, Opcode::Store(name));
            }
            AssignOp::BitOr => {
                self.chunk()
                    .insert(span.clone(), Opcode::Load(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Bor);
                self.chunk().insert(span, Opcode::Store(name));
            }
            AssignOp::Xor => {
                self.chunk()
                    .insert(span.clone(), Opcode::Load(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Xor);
                self.chunk().insert(span, Opcode::Store(name));
            }
        }
    }

    /// Performs generation of field assign
    pub fn gen_field_assign(
        &mut self,
        span: Span,
        container: Expr,
        name: String,
        op: AssignOp,
        value: Expr,
    ) {
        self.gen_expr(container);
        match op {
            AssignOp::Define => {
                self.gen_expr(value);
                self.chunk().insert(span, Opcode::DefineField(name));
            }
            AssignOp::Assign => {
                self.gen_expr(value);
                self.chunk().insert(span, Opcode::StoreField(name));
            }
            AssignOp::Add => {
                self.chunk().insert(span.clone(), Opcode::Dup);
                self.chunk()
                    .insert(span.clone(), Opcode::LoadField(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Add);
                self.chunk().insert(span, Opcode::StoreField(name));
            }
            AssignOp::Sub => {
                self.chunk().insert(span.clone(), Opcode::Dup);
                self.chunk()
                    .insert(span.clone(), Opcode::LoadField(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Sub);
                self.chunk().insert(span, Opcode::StoreField(name));
            }
            AssignOp::Mul => {
                self.chunk().insert(span.clone(), Opcode::Dup);
                self.chunk()
                    .insert(span.clone(), Opcode::LoadField(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Mul);
                self.chunk().insert(span, Opcode::StoreField(name));
            }
            AssignOp::Div => {
                self.chunk().insert(span.clone(), Opcode::Dup);
                self.chunk()
                    .insert(span.clone(), Opcode::LoadField(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Div);
                self.chunk().insert(span, Opcode::StoreField(name));
            }
            AssignOp::Mod => {
                self.chunk().insert(span.clone(), Opcode::Dup);
                self.chunk()
                    .insert(span.clone(), Opcode::LoadField(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Rem);
                self.chunk().insert(span, Opcode::StoreField(name));
            }
            AssignOp::BitAnd => {
                self.chunk().insert(span.clone(), Opcode::Dup);
                self.chunk()
                    .insert(span.clone(), Opcode::LoadField(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Band);
                self.chunk().insert(span, Opcode::StoreField(name));
            }
            AssignOp::BitOr => {
                self.chunk().insert(span.clone(), Opcode::Dup);
                self.chunk()
                    .insert(span.clone(), Opcode::LoadField(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Bor);
                self.chunk().insert(span, Opcode::StoreField(name));
            }
            AssignOp::Xor => {
                self.chunk().insert(span.clone(), Opcode::Dup);
                self.chunk()
                    .insert(span.clone(), Opcode::LoadField(name.clone()));
                self.gen_expr(value);
                self.chunk().insert(span.clone(), Opcode::Xor);
                self.chunk().insert(span, Opcode::StoreField(name));
            }
        }
    }

    /// Performs generation of assign
    pub fn gen_assign(&mut self, span: Span, what: Expr, op: AssignOp, value: Expr) {
        // Matching lhs
        match what {
            Expr::Variable { name, .. } => self.gen_variable_assign(span, name, op, value),
            Expr::Field {
                name, container, ..
            } => self.gen_field_assign(span, *container, name, op, value),
            _ => bug!("invalid assign lhs"),
        }
    }

    /// Performs generation of a statement
    pub fn gen_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::While {
                span,
                condition,
                block,
            } => self.gen_while(span, condition, block),
            Stmt::If {
                span,
                condition,
                then,
                else_,
            } => self.gen_if(span, condition, then, else_),
            Stmt::For {
                span,
                var,
                iterable,
                block,
            } => todo!(),
            Stmt::Class(class) => todo!(),
            Stmt::Enum(_) => todo!(),
            Stmt::Trait(_) => todo!(),
            Stmt::Function(function) => todo!(),
            Stmt::Assign {
                span,
                what,
                op,
                value,
            } => self.gen_assign(span, what, op, value),
            Stmt::Return { span, expr } => todo!(),
            Stmt::Continue(span) => todo!(),
            Stmt::Break(span) => todo!(),
            Stmt::Expr(expr) => self.gen_expr(expr),
            Stmt::Block(block) => todo!(),
            Stmt::Use { span, path, kind } => todo!(),
        }
    }
}
