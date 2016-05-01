#include <stdbool.h>
#include <string.h>
#include "pcb.h"

#define STR(s) (s), strlen(s)
#define STRl(s) (s ""), (sizeof(s) - 1)

int main() {
  pcb_Ctxt ctxt = pcb_Ctxt_new();

  pcb_FunctionType fun_ty = pcb_FunctionType_new(NULL, 0,
      pcb_Type_int(&ctxt, 32));
  pcb_FunctionRef foo = pcb_Function_create(&ctxt, STRl("foo"),
      pcb_FunctionType_clone(&fun_ty));
  pcb_BlockRef foo_start = pcb_Block_append(foo);
  pcb_ValueRef foo_ret =
      pcb_Block_build_const_int(foo_start, pcb_Type_int(&ctxt, 32), 0);
  pcb_Block_build_return(foo_start, foo_ret);

  pcb_FunctionRef main = pcb_Function_create(&ctxt, STRl("main"), fun_ty);
  pcb_BlockRef main_start = pcb_Block_append(main);
  pcb_BlockRef main_end = pcb_Block_append(main);
  pcb_ValueRef main_ret = pcb_Block_build_call(main_start, foo);
  pcb_Block_build_branch(main_start, main_end);
  pcb_Block_build_call(main_end, foo);
  pcb_Block_build_return(main_end, main_ret);

  pcb_Ctxt_print(&ctxt);
  pcb_Llvm_build_and_write(ctxt, STRl("test.o"), true);
}
