use {std, llvm, ty_};
use function::{Function, FuncContext};

pub struct Ctxt {
  type_ctxt: ty_::TypeContext,
  func_ctxt: FuncContext<'static, 'static>, // 'self, 'self
  _optimize: bool,
  target_machine: llvm::TargetMachine,
  target_data: llvm::TargetData,
}

impl Ctxt {
  pub fn new(opt: bool) -> Self {
    let opt_level = if opt {
      llvm::NoOptimization
    } else {
      llvm::DefaultOptimization
    };
    let target_machine = llvm::TargetMachine::new(opt_level).unwrap();
    let target_data = llvm::TargetData::from_target_machine(&target_machine);

    Ctxt {
      type_ctxt: ty_::TypeContext::new(),
      func_ctxt: FuncContext::new(),
      _optimize: opt,
      target_machine: target_machine,
      target_data: target_data,
    }
  }

  pub fn add_function<'a>(&'a self, name: &str, ty: ty_::Function<'a>)
      -> &'a mut Function<'a, 'a> {
    use std::mem::transmute;
    unsafe {
      transmute::<&mut Function, &mut Function>(
        self.func_ctxt.push(Function::new(name,
          transmute::<ty_::Function, ty_::Function>(ty))))
    }
  }

  pub fn type_int(&self, size: u32) -> &ty_::Type {
    self.type_ctxt.get(ty_::Type::new(ty_::TypeKind::Integer(size)))
  }

  pub fn build_and_write(self, output: &str, print_llvm_ir: bool) {
    let module = llvm::Module::new();

    let _optimizer = llvm::FnOptimizer::for_module(&module);

    for function in &self.func_ctxt {
      let llfunc = module.add_function(function.name(),
        llvm::get_function_type(&self.target_data, function.ty()));
      function.llvm().set(Some(llfunc));
    }
    for function in &self.func_ctxt {
      function.build();
    }

    if print_llvm_ir {
      module.dump();
    }

    module.verify();

    match self.target_machine.emit_to_file(&module, output) {
      Ok(()) => {},
      Err(e) => panic!("Failed to write to output file: {:?}", e),
    }
  }
}

impl std::fmt::Display for Ctxt {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    for func in &self.func_ctxt {
      try!(writeln!(f, "{}", func));
    }
    Ok(())
  }
}
