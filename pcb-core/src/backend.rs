use pcb::Ctxt;

// implement this for backend support
pub trait Backend {
  fn build_and_write(ctxt: Ctxt, output: &str, print_extra_info: bool);
}
