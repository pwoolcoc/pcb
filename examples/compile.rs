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
    let foo_ty = ty::Function::new(vec![], ctxt.type_int(32));
    let foo = ctxt.add_function("foo", foo_ty);
    let foo_start = foo.add_block();
    let foo_ret = foo_start.build_const_int(32, 0);
    foo_start.set_terminator_return(foo_ret);

    let main_ty = ty::Function::new(vec![], ctxt.type_int(32));
    let main = ctxt.add_function("main", main_ty);
    let main_start = main.add_block();
    let main_ret = main_start.build_call(foo);
    main_start.set_terminator_return(main_ret);
  }

  println!("{}", ctxt);
  ctxt.build_and_write("test.o", true);
}
