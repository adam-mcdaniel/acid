//! # Foreign Function Interface
//!
//! This module contains the definition of the foreign function interface (FFI) bindings, which
//! are used in the various stages of IR to represent calls to foreign functions.

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use serde_derive::{Serialize, Deserialize};

/// This is an FFI binding, which is used to call a foreign function in the virtual machine code.
///
/// The name is the symbol for the foreign function. The input cells is the number of cells that
/// the foreign function will read from the FFI channel. The output cells is the number of cells
/// that the foreign function will write to the FFI channel.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FFIBinding {
    pub name: String,
    pub input_cells: usize,
    pub output_cells: usize,
}

impl FFIBinding {
    /// Create a new FFI binding.
    pub fn new(name: String, input_cells: usize, output_cells: usize) -> Self {
        Self {
            name,
            input_cells,
            output_cells,
        }
    }
}

impl Display for FFIBinding {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.name)
    }
}

impl Debug for FFIBinding {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "ffi {}({}) -> {}",
            self.name, self.input_cells, self.output_cells
        )
    }
}
