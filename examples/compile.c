#include <stdbool.h>
#include <string.h>
#include "pcb.h"

int main() {
  pcb_Ctxt ctxt = pcb_Ctxt_new(false);

  pcb_FunctionType main_ty = pcb_FunctionType_new(NULL, 0,
      pcb_Ctxt_type_int(&ctxt, 32));
  pcb_FunctionRef foo = pcb_Ctxt_add_function(&ctxt, "foo", strlen("foo"),
      pcb_FunctionType_clone(&main_ty));
  pcb_BlockRef foo_start = pcb_Function_add_block(foo);
  pcb_Block_set_terminator_return(foo_start, pcb_Value_const_int(32, 1));

  pcb_FunctionRef main = pcb_Ctxt_add_function(&ctxt, "main", strlen("main"),
      main_ty);
  pcb_BlockRef main_start = pcb_Function_add_block(main);
  pcb_BlockRef main_end = pcb_Function_add_block(main);
  pcb_Block_set_terminator_branch(main_start, main_end);
  pcb_Block_set_terminator_return(main_end, pcb_Value_const_int(32, 3));

  //pcb_Ctxt_print(ctxt);
  pcb_Ctxt_build_and_write(ctxt, "test.o", strlen("test.o"), true);
}
