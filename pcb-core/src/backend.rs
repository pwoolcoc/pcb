use std;
use pcb::Ctxt;

// implement this for backend support
pub trait Backend {
  fn build_and_write<W>(ctxt: Ctxt, output: &mut W, print_extra_info: bool)
    where W: std::io::Write;
}
