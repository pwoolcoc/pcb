#![feature(pub_restricted)]
extern crate typed_arena;

mod pcb;
pub use pcb::Ctxt;
mod function;
pub use function::{Function, Value, Block};

mod ty_;
pub mod ty {
  pub use ty_::{Function, Type};
}

mod llvm;
mod common;

// the C API
pub mod pcb_c;
