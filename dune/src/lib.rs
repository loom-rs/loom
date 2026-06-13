/// Modules
pub mod errors;
mod eval;
pub mod frame;
pub mod ops;
pub mod refs;
pub mod value;

/// Imports
use crate::{
    frame::{Frame, Scope},
    ops::Chunk,
    refs::{MutRef, Ref},
    value::Module,
};
use common::{bug, io::IO, span::Span};

/// Defines module registry trait
pub trait ModuleRegistry {
    /// Resolves module
    fn resolve(&self, span: Span, path: &str) -> (String, Ref<Chunk>);

    /// Inserts module
    fn insert(&mut self, id: &str, module: MutRef<Module>);
}

/// Defines virtual machine
pub struct VirtualMachine<'io, 'reg> {
    /// Call stack
    pub(crate) stack: Vec<Frame>,

    /// IO
    pub io: &'io dyn IO,

    /// Module registry
    pub modules: &'reg mut dyn ModuleRegistry,

    /// Builtins scope
    pub(crate) builtins: MutRef<Scope>,
}

/// VM implementation
impl<'io, 'reg> VirtualMachine<'io, 'reg> {
    /// Creates new VM
    pub fn new(
        io: &'io dyn IO,
        modules: &'reg mut dyn ModuleRegistry,
        builtins: MutRef<Scope>,
    ) -> Self {
        Self {
            stack: Vec::new(),
            io,
            modules,
            builtins,
        }
    }

    /// Returns ref to current frame
    pub fn frame(&self) -> &Frame {
        self.stack
            .last()
            .unwrap_or_else(|| bug!("empty frames stack"))
    }

    /// Returns mut ref to current frame
    pub fn frame_mut(&mut self) -> &mut Frame {
        self.stack
            .last_mut()
            .unwrap_or_else(|| bug!("empty frames stack"))
    }

    /// Pushes new frame
    pub fn push(&mut self, chunk: Ref<Chunk>) {
        self.stack.push(Frame::new(chunk));
    }

    /// Pushes new frame with enclosing scope
    pub fn push_with_enclosing(&mut self, chunk: Ref<Chunk>, enclosing: MutRef<Scope>) {
        self.stack.push(Frame::with_enclosing(chunk, enclosing));
    }

    /// Pushes new frame with scope
    pub fn push_with_scope(&mut self, chunk: Ref<Chunk>, scope: MutRef<Scope>) {
        self.stack.push(Frame::with_scope(chunk, scope));
    }

    /// Pops top frame
    pub fn pop(&mut self) {
        self.stack.pop();
    }
}
