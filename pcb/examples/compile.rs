extern crate pcb;
use pcb::{Ctxt, Function, Block};
use pcb::ty::{self, Type};

/*
fn main() -> s32 {
  foo()
}
fn foo() -> s32 {
  0
}
*/

fn main() {
  let ctxt = Ctxt::new();

  {
    let s32_ty = Type::int(&ctxt, 32);
    let fun_ty = ty::Function::new(vec![], s32_ty);
    let foo = Function::new(&ctxt, "foo", fun_ty.clone());
    let foo_start = Block::append(foo);
    let foo_ret = foo_start.build_const_int(s32_ty, 0);
    foo_start.build_return(foo_ret);

    let main = Function::new(&ctxt, "main", fun_ty);
    let main_start = Block::append(main); // the first block added is the entry
                                          // block
    let main_end = Block::append(main);
    let main_ret = main_start.build_call(foo);
    main_start.build_branch(main_end);
    main_end.build_call(foo); // useless, but you can still do
    main_end.build_return(main_ret);
  }

  println!("{}", ctxt);
  ctxt.build_and_write("test.o", true);
}
