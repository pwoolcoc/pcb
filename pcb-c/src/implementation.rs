use super::{pcb_FunctionOpaque, pcb_FunctionRef, pcb_BlockOpaque, pcb_BlockRef,
  pcb_TypeOpaque, pcb_TypeRef, pcb_ValueOpaque, pcb_ValueRef, Wrap, Unwrap};
use pcb::{ty, Function, Block, Value};

use std::mem::transmute;

impl<'c> Wrap for Function<'c> {
  type Wrapped = pcb_FunctionOpaque;
  fn wrap(u: Self) -> pcb_FunctionRef {
    unsafe { transmute(u) }
  }
  fn wrap_slice<'a>(u: &'a [Self]) -> &'a [pcb_FunctionRef] {
    unsafe { transmute(u) }
  }
}
impl<'c> Unwrap<'c> for pcb_FunctionOpaque {
  type Unwrapped = Function<'c>;
  unsafe fn unwrap(w: pcb_FunctionRef) -> Function<'c> {
    transmute(w)
  }
  unsafe fn unwrap_slice(w: &[pcb_FunctionRef]) -> &[Function<'c>] {
    transmute(w)
  }
}

impl<'c> Wrap for Block<'c> {
  type Wrapped = pcb_BlockOpaque;
  fn wrap(u: Self) -> pcb_BlockRef {
    unsafe { transmute(u) }
  }
  fn wrap_slice<'a>(u: &'a [Self]) -> &'a [pcb_BlockRef] {
    unsafe { transmute(u) }
  }
}
impl<'c> Unwrap<'c> for pcb_BlockOpaque {
  type Unwrapped = Block<'c>;
  unsafe fn unwrap(w: pcb_BlockRef) -> Block<'c> {
    transmute(w)
  }
  unsafe fn unwrap_slice(w: &[pcb_BlockRef]) -> &[Block<'c>] {
    transmute(w)
  }
}

impl<'c> Wrap for Value<'c> {
  type Wrapped = pcb_ValueOpaque;
  fn wrap(u: Self) -> pcb_ValueRef {
    unsafe { transmute(u) }
  }
  fn wrap_slice<'a>(u: &'a [Self]) -> &'a [pcb_ValueRef] {
    unsafe { transmute(u) }
  }
}
impl<'c> Unwrap<'c> for pcb_ValueOpaque {
  type Unwrapped = Value<'c>;
  unsafe fn unwrap(w: pcb_ValueRef) -> Value<'c> {
    transmute(w)
  }
  unsafe fn unwrap_slice(w: &[pcb_ValueRef]) -> &[Value<'c>] {
    transmute(w)
  }
}

impl<'c> Wrap for ty::Type<'c> {
  type Wrapped = pcb_TypeOpaque;
  fn wrap(u: Self) -> pcb_TypeRef {
    unsafe { transmute(u) }
  }
  fn wrap_slice<'a>(u: &'a [Self]) -> &'a [pcb_TypeRef] {
    unsafe { transmute(u) }
  }
}
impl<'c> Unwrap<'c> for pcb_TypeOpaque {
  type Unwrapped = ty::Type<'c>;
  unsafe fn unwrap(w: pcb_TypeRef) -> ty::Type<'c> {
    transmute(w)
  }
  unsafe fn unwrap_slice(w: &[pcb_TypeRef]) -> &[ty::Type<'c>] {
    transmute(w)
  }
}
