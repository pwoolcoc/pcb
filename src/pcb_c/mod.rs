#![allow(non_camel_case_types, non_snake_case)]

// TODO(ubsan): SWITCH TO ABORT ON PANIC
// currently UBs if it panics

use std;
use {ty_, Ctxt, Value, Function, Block};

// Need to be destroyed
pub type pcb_Ctxt = *mut Ctxt;
pub type pcb_FunctionType = *mut ty_::Function<'static>;
// should be a ValueRef

// Do not need to be destroyed
pub type pcb_FunctionRef = *mut Function<'static>;
pub type pcb_BlockRef = *const Block;
pub type pcb_TypeRef = *const ty_::Type;
pub type pcb_ValueRef = *const Value;

// -- pcb_Ctxt --

#[no_mangle]
pub unsafe extern fn pcb_Ctxt_new(opt: bool) -> pcb_Ctxt {
  Box::into_raw(Box::new(Ctxt::new(opt)))
}
#[no_mangle]
pub unsafe extern fn pcb_Ctxt_delete(ctxt: pcb_Ctxt) {
  Box::from_raw(ctxt);
}
#[no_mangle]
pub unsafe extern fn pcb_Ctxt_build_and_write(ctxt: pcb_Ctxt, name: *const u8,
    name_len: usize, print_llvm_ir: bool) {
  let name = ptr_len_to_str(name, name_len);
  Box::from_raw(ctxt).build_and_write(name, print_llvm_ir)
}

#[no_mangle]
pub unsafe extern fn pcb_Ctxt_add_function(ctxt: *mut pcb_Ctxt,
    name: *const u8, name_len: usize, ty: pcb_FunctionType) -> pcb_FunctionRef {
  let name = ptr_len_to_str(name, name_len);
  &mut *(**ctxt).add_function(name, *Box::from_raw(ty))
}

#[no_mangle]
pub unsafe extern fn pcb_Ctxt_type_int(ctxt: *const pcb_Ctxt, size: u32)
    -> pcb_TypeRef {
  &*(**ctxt).type_int(size)
}

// -- pcb_FunctionType --

#[no_mangle]
pub unsafe extern fn pcb_FunctionType_new(inputs: *const pcb_TypeRef,
    inputs_len: usize, output: pcb_TypeRef) -> pcb_FunctionType {
  let inputs = if inputs_len == 0 {
    vec![]
  } else {
    let slice =
      std::slice::from_raw_parts(inputs as *const &ty_::Type, inputs_len);
    slice.to_owned()
  };
  Box::into_raw(Box::new(ty_::Function::new(inputs, &*output)))
}

#[no_mangle]
pub unsafe extern fn pcb_FunctionType_clone(ty: *const pcb_FunctionType)
    -> pcb_FunctionType {
  let ty = *ty;
  let ty = Box::from_raw(ty);
  let ret = ty.clone();
  Box::into_raw(ty);
  Box::into_raw(ret)
}

#[no_mangle]
pub unsafe extern fn pcb_FuntionType_delete(func: pcb_FunctionType) {
  Box::from_raw(func);
}

// -- pcb_FunctionRef --

#[no_mangle]
pub unsafe extern fn pcb_Function_add_block(func: pcb_FunctionRef)
    -> pcb_BlockRef {
  &*(*func).add_block()
}

// -- pcb_BlockRef --

#[no_mangle]
pub unsafe extern fn pcb_Block_set_terminator_branch(blk: pcb_BlockRef,
    to: pcb_BlockRef) {
  (*blk).set_terminator_branch(&*to);
}

#[no_mangle]
pub unsafe extern fn pcb_Block_build_const_int(blk: pcb_BlockRef, size: u32,
    value: u64) -> pcb_ValueRef {
  (*blk).build_const_int(size, value)
}

// takes ownership of val
#[no_mangle]
pub unsafe extern fn pcb_Block_set_terminator_return(blk: pcb_BlockRef,
    val: pcb_ValueRef) {
  (*blk).set_terminator_return(&*val);
}

// implementation functions
unsafe fn ptr_len_to_str<'a>(ptr: *const u8, len: usize) -> &'a str {
  if len == 0 {
    ""
  } else {
    std::str::from_utf8(std::slice::from_raw_parts(ptr, len)).unwrap()
  }
}
