#![allow(non_camel_case_types)]

// TODO(ubsan): SWITCH TO ABORT ON PANIC
// currently UBs if it panics
extern crate pcb;
extern crate pcb_llvm;
extern crate libc;

use pcb::{ty, Ctxt, Function, Block};

mod implementation;

// Need to be destroyed
#[repr(C)]
pub struct pcb_CtxtOpaque(Ctxt);
pub type pcb_Ctxt = *mut pcb_CtxtOpaque;
#[repr(C)]
pub struct pcb_FunctionTypeOpaque(ty::Function<'static>);
pub type pcb_FunctionType = *mut pcb_FunctionTypeOpaque;

// Do not need to be destroyed
#[repr(C)]
pub struct pcb_FunctionOpaque(());
pub type pcb_FunctionRef = *const pcb_FunctionOpaque;
#[repr(C)]
pub struct pcb_BlockOpaque(());
pub type pcb_BlockRef = *const pcb_BlockOpaque;
#[repr(C)]
pub struct pcb_ValueOpaque(());
pub type pcb_ValueRef = *const pcb_ValueOpaque;
#[repr(C)]
pub struct pcb_TypeOpaque(());
pub type pcb_TypeRef = *const pcb_TypeOpaque;

// == pcb_Ctxt ==

#[no_mangle]
pub unsafe extern fn pcb_ctxt() -> pcb_Ctxt {
  Box::into_raw(Box::new(pcb_CtxtOpaque(Ctxt::new())))
}
#[no_mangle]
pub unsafe extern fn pcb_delete_ctxt(ctxt: pcb_Ctxt) {
  Box::from_raw(ctxt);
}
#[no_mangle]
pub unsafe extern fn pcb_print_ctxt(ctxt: *const pcb_Ctxt) {
  println!("{}", (**ctxt).0);
}

// == pcb_FunctionType ==

#[no_mangle]
pub unsafe extern fn pcb_function_type(mut inputs: *const pcb_TypeRef,
    inputs_len: libc::size_t, output: pcb_TypeRef) -> pcb_FunctionType {
  let inputs = if inputs_len == 0 {
    vec![]
  } else {
    let mut v = vec![];
    let end = inputs.offset(inputs_len as isize);
    while inputs != end {
      v.push(unwrap(*inputs));
      inputs = inputs.offset(1);
    }
    v
  };
  Box::into_raw(Box::new(pcb_FunctionTypeOpaque(
      ty::Function::new(inputs, unwrap(output)))))
}

#[no_mangle]
pub unsafe extern fn pcb_clone_function_type(ty: *const pcb_FunctionType)
    -> pcb_FunctionType {
  let ref_ = &(**ty).0;
  Box::into_raw(Box::new(pcb_FunctionTypeOpaque(ref_.clone())))
}

#[no_mangle]
pub unsafe extern fn pcb_delete_function_type(func: pcb_FunctionType) {
  Box::from_raw(func);
}

// == pcb_FunctionRef ==

#[no_mangle]
pub unsafe extern fn pcb_add_function(ctxt: *const pcb_Ctxt,
    name: *const libc::c_char, name_len: libc::size_t, ty: pcb_FunctionType)
    -> pcb_FunctionRef {
  let name = ptr_len_to_str(name as *const u8, name_len);
  wrap(Function::new(&(**ctxt).0, name, Box::from_raw(ty).0))
}

#[no_mangle]
pub unsafe extern fn pcb_get_argument(func: pcb_FunctionRef,
    number: u32) -> pcb_ValueRef {
  wrap(unwrap(func).get_argument(number))
}

// == pcb_BlockRef ==

#[no_mangle]
pub unsafe extern fn pcb_append_block(func: pcb_FunctionRef) -> pcb_BlockRef {
  wrap(Block::append(unwrap(func)))
}

// misc
#[no_mangle]
pub unsafe extern fn pcb_build_const_int(blk: pcb_BlockRef,
    ty: pcb_TypeRef, value: u64) -> pcb_ValueRef {
  wrap(unwrap(blk).build_const_int(unwrap(ty), value))
}

#[no_mangle]
pub unsafe extern fn pcb_build_call(blk: pcb_BlockRef,
    func: pcb_FunctionRef, args: *const pcb_ValueRef, args_len: libc::size_t)
    -> pcb_ValueRef {
  let opaque = ptr_len_to_slice(args, args_len);
  let mut unwrapped = vec![];
  for el in opaque {
    unwrapped.push(unwrap(*el));
  }
  wrap(unwrap(blk).build_call(unwrap(func), &unwrapped))
}

// binops
#[no_mangle]
pub unsafe extern fn pcb_build_mul(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_mul(unwrap(lhs), unwrap(rhs)))
}
#[no_mangle]
pub unsafe extern fn pcb_build_udiv(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_udiv(unwrap(lhs), unwrap(rhs)))
}
#[no_mangle]
pub unsafe extern fn pcb_build_sdiv(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_sdiv(unwrap(lhs), unwrap(rhs)))
}
#[no_mangle]
pub unsafe extern fn pcb_build_urem(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_urem(unwrap(lhs), unwrap(rhs)))
}
#[no_mangle]
pub unsafe extern fn pcb_build_srem(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_srem(unwrap(lhs), unwrap(rhs)))
}

#[no_mangle]
pub unsafe extern fn pcb_build_add(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_add(unwrap(lhs), unwrap(rhs)))
}
#[no_mangle]
pub unsafe extern fn pcb_build_sub(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_sub(unwrap(lhs), unwrap(rhs)))
}

#[no_mangle]
pub unsafe extern fn pcb_build_shl(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_shl(unwrap(lhs), unwrap(rhs)))
}
#[no_mangle]
pub unsafe extern fn pcb_build_zshr(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_zshr(unwrap(lhs), unwrap(rhs)))
}
#[no_mangle]
pub unsafe extern fn pcb_build_sshr(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_sshr(unwrap(lhs), unwrap(rhs)))
}

#[no_mangle]
pub unsafe extern fn pcb_build_and(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_and(unwrap(lhs), unwrap(rhs)))
}
#[no_mangle]
pub unsafe extern fn pcb_build_xor(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_xor(unwrap(lhs), unwrap(rhs)))
}
#[no_mangle]
pub unsafe extern fn pcb_build_or(blk: pcb_BlockRef, lhs: pcb_ValueRef,
    rhs: pcb_ValueRef) -> pcb_ValueRef {
  wrap(unwrap(blk).build_or(unwrap(lhs), unwrap(rhs)))
}

// terminators
#[no_mangle]
pub unsafe extern fn pcb_build_branch(blk: pcb_BlockRef,
    to: pcb_BlockRef) {
  unwrap(blk).build_branch(unwrap(to));
}

#[no_mangle]
pub unsafe extern fn pcb_build_return(blk: pcb_BlockRef,
    val: pcb_ValueRef) {
  unwrap(blk).build_return(unwrap(val))
}

// == pcb_TypeRef ==

#[no_mangle]
pub unsafe extern fn pcb_int_type(ctxt: *const pcb_Ctxt, size: u32)
    -> pcb_TypeRef {
  wrap(ty::Type::int(&(**ctxt).0, size))
}

// == pcb_llvm ==

#[no_mangle]
pub unsafe extern fn pcb_llvm_build_and_write(ctxt: pcb_Ctxt,
    name: *const libc::c_char, name_len: usize, print_llvm_ir: bool) {
  let name = ptr_len_to_str(name as *const u8, name_len);
  Box::from_raw(ctxt).0.build_and_write::<pcb_llvm::Llvm>(name, print_llvm_ir)
}


// implementation functions
unsafe fn ptr_len_to_str(ptr: *const u8, len: libc::size_t) -> &'static str {
  if len == 0 {
    ""
  } else {
    std::str::from_utf8(std::slice::from_raw_parts(ptr, len)).unwrap()
  }
}

unsafe fn ptr_len_to_slice<T>(ptr: *const T, len: libc::size_t)
    -> &'static [T] {
  if len == 0 {
    &[]
  } else {
    std::slice::from_raw_parts(ptr, len)
  }
}

fn wrap<T: Wrap>(w: T) -> *const T::Wrapped {
  Wrap::wrap(w)
}
unsafe fn unwrap<'c, T: Unwrap<'c>>(u: *const T) -> T::Unwrapped {
  Unwrap::unwrap(u)
}

trait Wrap: Sized {
  type Wrapped;
  fn wrap(u: Self) -> *const Self::Wrapped;
}
trait Unwrap<'c>: Sized {
  type Unwrapped: Wrap<Wrapped = Self>;
  unsafe fn unwrap(w: *const Self) -> Self::Unwrapped;
}
