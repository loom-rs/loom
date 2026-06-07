/// Modules
mod errors;
mod eval;
mod frame;
mod ops;
mod refs;
mod value;

/// Imports
use crate::frame::Frame;

/// Defines virtual machine
pub struct VirtualMachine {
    /// Call stack
    stack: Vec<Frame>,
}
