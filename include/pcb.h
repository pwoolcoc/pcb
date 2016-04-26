#ifndef PCB_H
#define PCB_H

#include <stdbool.h>
#include <stdlib.h>
#include <stdint.h>

// Must be given back to pcb in some way
// Either with functions that take ownership or to a "pcb_destroy_*" function
typedef struct pcb_CtxtOpaque* pcb_Ctxt;
typedef struct pcb_ValueOpaque* pcb_Value;
typedef struct pcb_FunctionTypeOpaque* pcb_FunctionType;

typedef struct pcb_FunctionOpaque* pcb_FunctionRef;
typedef struct pcb_BlockOpaque* pcb_BlockRef;
typedef struct pcb_TypeOpaqua* pcb_TypeRef;


// -- pcb_Ctxt --

pcb_Ctxt pcb_Ctxt_new(bool opt);
void pcb_Ctxt_delete(pcb_Ctxt ctxt);
/// Takes ownership of ctxt
void pcb_Ctxt_build_and_write(pcb_Ctxt ctxt,
    char const* name, size_t name_len, bool print_llvm_ir);

pcb_FunctionRef pcb_Ctxt_add_function(pcb_Ctxt* ctxt,
    char const* name, size_t name_len, pcb_FunctionType ty);

pcb_TypeRef pcb_Ctxt_type_int(pcb_Ctxt const* ctxt, uint32_t size);

// -- pcb_FunctionType --

pcb_FunctionType pcb_FunctionType_new(pcb_TypeRef const* inputs,
    size_t inputs_len, pcb_TypeRef output);
pcb_FunctionType pcb_FunctionType_clone(pcb_FunctionType const* ty);
void pcb_FunctionType_delete(pcb_FunctionType func);

// -- pcb_FunctionRef --

pcb_BlockRef pcb_Function_add_block(pcb_FunctionRef func);

// -- pcb_Value --

pcb_Value pcb_Value_const_int(uint32_t size, uint64_t value);

// -- pcb_BlockRef --

void pcb_Block_set_terminator_branch(pcb_BlockRef blk, pcb_BlockRef to);
/// takes ownership of value
void pcb_Block_set_terminator_return(pcb_BlockRef blk, pcb_Value value);


#endif
