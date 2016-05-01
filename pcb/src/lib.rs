extern crate pcb_core as core;

pub struct Ctxt(core::pcb::Ctxt);

impl std::fmt::Display for Ctxt {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    (self.0).fmt(f)
  }
}


impl Ctxt {
  pub fn new() -> Ctxt {
    Ctxt(core::pcb::Ctxt::new(false))
  }

  pub fn build_and_write<B>(self, output_file: &str, print_extra_info: bool)
      where B: core::backend::Backend {
    B::build_and_write(self.0, output_file, print_extra_info)
  }
}

#[derive(Copy, Clone)]
pub struct Function<'c>(&'c core::function::Function<'c>);

impl<'c> Function<'c> {
  pub fn new(ctxt: &'c Ctxt, name: &str, ty: ty::Function<'c>) -> Self {
    Function(ctxt.0.add_function(name, ty.inner()))
  }
}

#[derive(Copy, Clone)]
pub struct Block<'c>(&'c core::function::Block<'c>);

macro_rules! chk_term {
  ($this:expr) => (
    assert!($this.0.terminator.get().is_none(), "pcb_assert: \
      attempt to build instruction after a terminator");
  )
}

impl<'c> Block<'c> {
  pub fn append(func: Function<'c>) -> Self {
    Block(func.0.add_block())
  }

  pub fn build_const_int(self, ty: ty::Type<'c>, value: u64) -> Value<'c> {
    chk_term!(self);
    Value(self.0.add_value(
        core::function::ValueKind::ConstInt { ty: ty.inner(), value: value }))
  }
  pub fn build_call(self, func: Function<'c>) -> Value<'c> {
    chk_term!(self);
    Value(self.0.add_value(
        core::function::ValueKind::Call(func.0)))
  }

  pub fn build_return(self, value: Value<'c>) {
    chk_term!(self);
    self.0.terminator.set(core::function::Terminator::Return(value.0))
  }
  pub fn build_branch(self, blk: Block<'c>) {
    chk_term!(self);
    self.0.terminator.set(core::function::Terminator::Branch(blk.0))
  }
}

pub struct Value<'c>(&'c core::function::Value<'c>);

trait TyExt {
  type Output;
  fn inner(self) -> Self::Output;
  fn inner_ref(&self) -> &Self::Output;
}

pub mod ty {
  use core::ty;
  use super::Ctxt;
  #[derive(Copy, Clone, PartialEq, Eq, Hash)]
  pub struct Type<'c>(&'c ty::TypeKind);

  impl<'c> Type<'c> {
    pub fn int(ctxt: &Ctxt, size: u32) -> Type {
      Type(ctxt.0.get_type(ty::TypeKind::Integer(size)))
    }
  }

  #[derive(Clone)]
  pub struct Function<'c>(ty::Function<'c>);

  impl<'c> Function<'c> {
    pub fn new(_inputs: Vec<Type<'c>>, output: Type<'c>) -> Function<'c> {
      Function(ty::Function {
        output: output.0,
      })
    }

    #[inline(always)]
    pub fn output(&self) -> Type<'c> {
      Type(self.0.output)
    }
  }

  impl<'c> super::TyExt for Type<'c> {
    type Output = &'c ty::TypeKind;
    fn inner(self) -> &'c ty::TypeKind { self.0 }
    fn inner_ref(&self) -> &&'c ty::TypeKind { &self.0 }
  }
  impl<'c> super::TyExt for Function<'c> {
    type Output = ty::Function<'c>;
    fn inner(self) -> ty::Function<'c> { self.0 }
    fn inner_ref(&self) -> &ty::Function<'c> { &self.0 }
  }
}
