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
  x + x
}
 */

int main() {
  pcb_Ctxt ctxt = pcb_Ctxt_new();

  pcb_TypeRef ty_inputs[] = {pcb_Type_int(&ctxt, 32)};
  pcb_FunctionType foo_ty = pcb_FunctionType_new(SLICE(ty_inputs),
      pcb_Type_int(&ctxt, 32));
  pcb_FunctionRef foo = pcb_Function_create(&ctxt, STRl("foo"), foo_ty);
  pcb_BlockRef foo_start = pcb_Block_append(foo);
  pcb_ValueRef lhs = pcb_Function_get_argument(foo, 0);
  pcb_ValueRef rhs =
    pcb_Block_build_const_int(foo_start, pcb_Type_int(&ctxt, 32), 5);
  pcb_ValueRef foo_ret = pcb_Block_build_add(foo_start, lhs, rhs);
  pcb_Block_build_return(foo_start, foo_ret);

  pcb_FunctionType main_ty = pcb_FunctionType_new(NULL, 0,
      pcb_Type_int(&ctxt, 32));
  pcb_FunctionRef main = pcb_Function_create(&ctxt, STRl("main"), main_ty);
  pcb_BlockRef main_start = pcb_Block_append(main);

  pcb_ValueRef inputs[] = {
    pcb_Block_build_const_int(main_start, pcb_Type_int(&ctxt, 32), 0)};
  pcb_ValueRef main_ret = pcb_Block_build_call(main_start, foo, SLICE(inputs));
  pcb_Block_build_return(main_start, main_ret);

  pcb_Ctxt_print(&ctxt);
  pcb_Llvm_build_and_write(ctxt, STRl("test.o"), true);
}
