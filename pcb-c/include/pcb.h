
#ifndef cheddar_generated_pcb_h
#define cheddar_generated_pcb_h


#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>



typedef struct pcb_CtxtOpaque pcb_CtxtOpaque;

typedef pcb_CtxtOpaque* pcb_Ctxt;

typedef struct pcb_FunctionTypeOpaque pcb_FunctionTypeOpaque;

typedef pcb_FunctionTypeOpaque* pcb_FunctionType;

typedef struct pcb_FunctionOpaque pcb_FunctionOpaque;

typedef pcb_FunctionOpaque const* pcb_FunctionRef;

typedef struct pcb_BlockOpaque pcb_BlockOpaque;

typedef pcb_BlockOpaque const* pcb_BlockRef;

typedef struct pcb_ValueOpaque pcb_ValueOpaque;

typedef pcb_ValueOpaque const* pcb_ValueRef;

typedef struct pcb_TypeOpaque pcb_TypeOpaque;

typedef pcb_TypeOpaque const* pcb_TypeRef;

pcb_Ctxt pcb_Ctxt_new(void);

void pcb_Ctxt_delete(pcb_Ctxt ctxt);

void pcb_Ctxt_print(pcb_Ctxt const* ctxt);

pcb_FunctionType pcb_FunctionType_new(pcb_TypeRef const* inputs, size_t inputs_len, pcb_TypeRef output);

pcb_FunctionType pcb_FunctionType_clone(pcb_FunctionType const* ty);

void pcb_FuntionType_delete(pcb_FunctionType func);

pcb_FunctionRef pcb_Function_create(pcb_Ctxt const* ctxt, char const* name, size_t name_len, pcb_FunctionType ty);

pcb_BlockRef pcb_Block_append(pcb_FunctionRef func);

pcb_ValueRef pcb_Block_build_const_int(pcb_BlockRef blk, pcb_TypeRef ty, uint64_t value);

pcb_ValueRef pcb_Block_build_call(pcb_BlockRef blk, pcb_FunctionRef func);

void pcb_Block_build_branch(pcb_BlockRef blk, pcb_BlockRef to);

void pcb_Block_build_return(pcb_BlockRef blk, pcb_ValueRef val);

pcb_TypeRef pcb_Type_int(pcb_Ctxt const* ctxt, uint32_t size);

void pcb_Llvm_build_and_write(pcb_Ctxt ctxt, char const* name, uintptr_t name_len, bool print_llvm_ir);



#ifdef __cplusplus
}
#endif


#endif
