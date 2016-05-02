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
    use function::{Value, ValueKind, ValueContext, BlockContext};

    let ret = unsafe {
      let ret = self.func_ctxt.push(Function {
        name: name.to_owned(),
        ty: transmute::<ty::Function<'c>, ty::Function<'static>>(ty),
        values: ValueContext::new(),
        blocks: BlockContext::new(),
      });
      transmute::<&'c Function<'static>, &'c Function<'c>>(ret)
    };
    for param_ty in &ret.ty.inputs[..] {
      ret.values.push(Value {
        number: ret.values.len() as u32,
        kind: ValueKind::Parameter(param_ty),
        func: ret,
      });
    }
    ret
  }

  pub fn get_type(&self, ty: ty::Type) -> &ty::Type {
    self.type_ctxt.get(ty)
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
