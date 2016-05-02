use {std, ty};
use function::{Function, FuncContext};

pub struct Ctxt {
  pub type_ctxt: ty::TypeContext,
  pub func_ctxt: FuncContext<'static>, // 'self
  pub optimize: bool,
}

impl Ctxt {
  pub fn new(opt: bool) -> Self {
    Ctxt {
      type_ctxt: ty::TypeContext::new(),
      func_ctxt: FuncContext::new(),
      optimize: opt,
    }
  }

  pub fn add_function<'c>(&'c self, name: &str, ty: ty::Function<'c>)
      -> &'c Function<'c> {
    use std::mem::transmute;
    unsafe {
      transmute::<&'c Function<'static>, &'c Function<'c>>(
        self.func_ctxt.push(Function::new(name,
          transmute::<ty::Function<'c>, ty::Function<'static>>(ty))))
    }
  }

  pub fn get_type(&self, kind: ty::TypeKind) -> &ty::TypeKind {
    self.type_ctxt.get(kind)
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
