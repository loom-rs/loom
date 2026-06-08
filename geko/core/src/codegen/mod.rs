use common::{bug, span::Span};
/// Imports
use dune::{
    ops::{Chunk, Opcode},
    refs::Ref,
    value::Value,
};

use crate::ast::{BinOp, Block, Expr, Lit, Stmt, UnOp};

/// Defines loop labels information
pub struct LoopLabels {
    /// Start label
    start_label: usize,

    /// End label
    end_label: usize,
}

/// Defines bytecode generator
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

    /// Performs generation of expression
    pub fn gen_expr(&mut self, expr: Expr) {
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
            Expr::Call { span, args, what } => todo!(),
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

    /// Performs generation of a while
    pub fn gen_while(&mut self, span: Span, cond: Expr, block: Block) {}

    /// Performs generation of a statement
    pub fn gen_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::While {
                span,
                condition,
                block,
            } => todo!(),
            Stmt::If {
                span,
                condition,
                then,
                else_,
            } => todo!(),
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
                name,
                op,
                value,
            } => todo!(),
            Stmt::Set {
                span,
                container,
                name,
                op,
                value,
            } => todo!(),
            Stmt::Return { span, expr } => todo!(),
            Stmt::Continue(span) => todo!(),
            Stmt::Break(span) => todo!(),
            Stmt::Expr(expr) => todo!(),
            Stmt::Scope(block) => todo!(),
            Stmt::Use { span, path, kind } => todo!(),
        }
    }
}
