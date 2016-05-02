#include <stdbool.h>
#include <string.h>
#include "pcb.h"

#define STR(s) (s), strlen(s)
#define STRl(s) (s ""), (sizeof(s) - 1)
#define SLICE(a) (a), (sizeof(a) / sizeof(a[0]))

/*
fn main() -> s32 {
  fib(0)
}
fn fib(x: s32) -> s32 {
  (x + x) * 3
}
 */

int main() {
  pcb_Ctxt ctxt = pcb_ctxt();

  pcb_TypeRef ty_inputs[] = {pcb_int_type(&ctxt, 32)};
  pcb_FunctionType foo_ty = pcb_function_type(SLICE(ty_inputs),
      pcb_int_type(&ctxt, 32));
  pcb_FunctionRef foo = pcb_add_function(&ctxt, STRl("foo"), foo_ty);
  pcb_BlockRef foo_start = pcb_append_block(foo);
  pcb_ValueRef lhs_inner = pcb_get_argument(foo, 0);
  pcb_ValueRef lhs = pcb_build_add(foo_start, lhs_inner, lhs_inner);
  pcb_ValueRef rhs =
    pcb_build_const_int(foo_start, pcb_int_type(&ctxt, 32), 3);
  pcb_ValueRef foo_ret = pcb_build_mul(foo_start, lhs, rhs);
  pcb_build_return(foo_start, foo_ret);

  pcb_FunctionType main_ty = pcb_function_type(NULL, 0,
      pcb_int_type(&ctxt, 32));
  pcb_FunctionRef main = pcb_add_function(&ctxt, STRl("main"), main_ty);
  pcb_BlockRef main_start = pcb_append_block(main);

  pcb_ValueRef inputs[] = {
    pcb_build_const_int(main_start, pcb_int_type(&ctxt, 32), 7)};
  pcb_ValueRef main_ret = pcb_build_call(main_start, foo, SLICE(inputs));
  pcb_build_return(main_start, main_ret);

  pcb_print_ctxt(&ctxt);
  pcb_llvm_build_and_write(ctxt, STRl("test.o"), true);
}
