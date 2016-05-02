
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

pcb_Ctxt pcb_ctxt(void);

void pcb_delete_ctxt(pcb_Ctxt ctxt);

void pcb_print_ctxt(pcb_Ctxt const* ctxt);

pcb_FunctionType pcb_function_type(pcb_TypeRef const* inputs, size_t inputs_len, pcb_TypeRef output);

pcb_FunctionType pcb_clone_function_type(pcb_FunctionType const* ty);

void pcb_delete_function_type(pcb_FunctionType func);

pcb_FunctionRef pcb_add_function(pcb_Ctxt const* ctxt, char const* name, size_t name_len, pcb_FunctionType ty);

pcb_ValueRef pcb_get_argument(pcb_FunctionRef func, uint32_t number);

pcb_BlockRef pcb_append_block(pcb_FunctionRef func);

pcb_ValueRef pcb_build_const_int(pcb_BlockRef blk, pcb_TypeRef ty, uint64_t value);

pcb_ValueRef pcb_build_call(pcb_BlockRef blk, pcb_FunctionRef func, pcb_ValueRef const* args, size_t args_len);

pcb_ValueRef pcb_build_mul(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_udiv(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_sdiv(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_urem(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_srem(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_add(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_sub(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_shl(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_zshr(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_sshr(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_and(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_xor(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

pcb_ValueRef pcb_build_or(pcb_BlockRef blk, pcb_ValueRef lhs, pcb_ValueRef rhs);

void pcb_build_branch(pcb_BlockRef blk, pcb_BlockRef to);

void pcb_build_return(pcb_BlockRef blk, pcb_ValueRef val);

pcb_TypeRef pcb_int_type(pcb_Ctxt const* ctxt, uint32_t size);

void pcb_llvm_build_and_write(pcb_Ctxt ctxt, char const* name, uintptr_t name_len, bool print_llvm_ir);



#ifdef __cplusplus
}
#endif


#endif
