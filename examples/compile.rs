extern crate pcb;
use pcb::Ctxt;
use pcb::ty;

/*
fn main() -> s32 {
  foo()
}
fn foo() -> s32 {
  0
}
*/

fn main() {
  let ctxt = Ctxt::new(false);

  {
    let fun_ty = ty::Function::new(vec![], ctxt.type_int(32));
    let foo = ctxt.add_function("foo", fun_ty.clone());
    let foo_start = foo.add_block();
    let foo_ret = foo_start.build_const_int(ctxt.type_int(32), 0);
    foo_start.set_terminator_return(foo_ret);

    let main = ctxt.add_function("main", fun_ty);
    let main_start = main.add_block(); // the first block added is the entry
                                       // block
    let main_end = main.add_block();
    let main_ret = main_start.build_call(foo);
    main_start.set_terminator_branch(main_end);
    main_end.build_call(foo); // useless, but you can still do
    main_end.set_terminator_return(main_ret);
  }

  println!("{}", ctxt);
  ctxt.build_and_write("test.o", true);
}
