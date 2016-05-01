extern crate pcb_core as core;
use core::pcb::Ctxt;
use core::backend::Backend;

pub struct Llvm;

impl Backend for Llvm {
  fn build_and_write(ctxt: Ctxt, output: &str, print_llvm_ir: bool) {
    let module = core::llvm::Module::new();

    let _optimizer = core::llvm::FnOptimizer::for_module(&module);

    for function in &ctxt.func_ctxt {
      let llfunc = module.add_function(function.name(),
        core::llvm::get_function_type(&ctxt.target_data, function.ty()));
      function.llvm.set(Some(llfunc));
    }
    for function in &ctxt.func_ctxt {
      function.build();
    }

    if print_llvm_ir {
      module.dump();
    }

    module.verify();

    match ctxt.target_machine.emit_to_file(&module, output) {
      Ok(()) => {},
      Err(e) => panic!("Failed to write to output file: {:?}", e),
    }
  }
}
