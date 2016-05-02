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

  pub fn get_argument(&self, number: u32) -> Value<'c> {
    assert!(number < self.0.ty.inputs.len() as u32, "pcb_assert: attempted to \
        get nonexistent argument");
    Value(self.0.values.get(number as usize).unwrap())
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

macro_rules! chk_op_types {
  ($lhs:expr, $rhs:expr) => (
    assert!($lhs.0.ty() == $rhs.0.ty(), "pcb_assert: lhs and rhs are not of \
      the same type");
    /*if let core::ty::Type::Integer(_) = *lhs.0.ty() {
    } else {
      panic!("pcb_assert: `add` must take values of integer type");
    }*/
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
  pub fn build_call(self, func: Function<'c>, args: &[Value<'c>])
      -> Value<'c> {
    chk_term!(self);
    assert!(args.len() == func.0.ty.inputs.len(), "pcb_assert: attempt to call \
      a function with the incorrect number of arguments");
    for (arg, param_ty) in args.iter().zip(func.0.ty.inputs.iter()) {
      assert!(arg.0.ty() == *param_ty, "pcb_assert: attempt to call a function \
        with incorrect argument types");
    }
    // TODO(ubsan): check calls against type of func
    let mut inner_params = vec![];
    for param in args {
      inner_params.push(param.0)
    }
    Value(self.0.add_value(
      core::function::ValueKind::Call { function: func.0,
        parameters: inner_params.into_boxed_slice() }))
  }

  // -- binops --
  pub fn build_mul(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::Mul(lhs.0, rhs.0)))
  }
  pub fn build_udiv(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::UDiv(lhs.0, rhs.0)))
  }
  pub fn build_sdiv(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::SDiv(lhs.0, rhs.0)))
  }
  pub fn build_urem(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::URem(lhs.0, rhs.0)))
  }
  pub fn build_srem(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::SRem(lhs.0, rhs.0)))
  }

  pub fn build_add(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::Add(lhs.0, rhs.0)))
  }
  pub fn build_sub(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::Sub(lhs.0, rhs.0)))
  }

  pub fn build_shl(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::Shl(lhs.0, rhs.0)))
  }
  pub fn build_zshr(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::ZShr(lhs.0, rhs.0)))
  }
  pub fn build_sshr(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::SShr(lhs.0, rhs.0)))
  }

  pub fn build_and(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::And(lhs.0, rhs.0)))
  }
  pub fn build_xor(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::Xor(lhs.0, rhs.0)))
  }
  pub fn build_or(self, lhs: Value<'c>, rhs: Value<'c>) -> Value<'c> {
    chk_term!(self);
    chk_op_types!(lhs, rhs);
    Value(self.0.add_value(core::function::ValueKind::Or(lhs.0, rhs.0)))
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

#[derive(Copy, Clone)]
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
  pub struct Type<'c>(&'c ty::Type);

  impl<'c> Type<'c> {
    pub fn int(ctxt: &Ctxt, size: u32) -> Type {
      Type(ctxt.0.get_type(ty::Type::Integer(size)))
    }
  }

  #[derive(Clone)]
  pub struct Function<'c>(ty::Function<'c>);

  impl<'c> Function<'c> {
    pub fn new(inputs: Vec<Type<'c>>, output: Type<'c>) -> Function<'c> {
      let mut input_inner = vec![];
      for input in inputs {
        input_inner.push(input.0);
      }
      Function(ty::Function {
        inputs: input_inner.into_boxed_slice(),
        output: output.0,
      })
    }

    #[inline(always)]
    pub fn output(&self) -> Type<'c> {
      Type(self.0.output)
    }
  }

  impl<'c> super::TyExt for Type<'c> {
    type Output = &'c ty::Type;
    fn inner(self) -> &'c ty::Type { self.0 }
    fn inner_ref(&self) -> &&'c ty::Type { &self.0 }
  }
  impl<'c> super::TyExt for Function<'c> {
    type Output = ty::Function<'c>;
    fn inner(self) -> ty::Function<'c> { self.0 }
    fn inner_ref(&self) -> &ty::Function<'c> { &self.0 }
  }
}
