extern crate pcb_core as core;

use core::pcb::Ctxt;
use core::backend::Backend;
use core::function::{Block, Function, Value, Terminator};

use std::collections::HashMap;

mod llvm;

pub struct Llvm;

impl Backend for Llvm {
  fn build_and_write<W>(ctxt: Ctxt, output: &mut W, print_llvm_ir: bool)
      where W: std::io::Write {
    let module = llvm::Module::new();
    let mut function_hm = HashMap::new();
    let opt_level = if ctxt.optimize {
      llvm::NoOptimization
    } else {
      llvm::DefaultOptimization
    };
    let target_machine = llvm::TargetMachine::new(opt_level).unwrap();
    let target_data = llvm::TargetData::from_target_machine(&target_machine);

    //let _optimizer = llvm::FnOptimizer::for_module(&module);

    for function in &ctxt.func_ctxt {
      function_hm.insert(function,
        module.add_function(&function.name,
          llvm::get_function_type(&target_data, function.ty())));
    }
    for function in &ctxt.func_ctxt {
      build_function(function,
        *function_hm.get(&function).expect("pcb_ice: blorghle"), &function_hm);
    }

    if print_llvm_ir {
      module.dump();
    }

    module.verify();

    match target_machine.emit_to(&module, output) {
      Ok(()) => {},
      Err(e) => panic!("Failed to write to output file: {:?}", e),
    }
  }
}

fn build_function<'a>(func: &Function<'a>, llfunc: llvm::Value,
    functions: &HashMap<&Function<'a>, llvm::Value>) {
  let mut llvm_blocks = vec![];
  let mut llvm_values = vec![];

  if func.blocks.iter().next().is_none() {
    panic!("pcb_assert: function {} has no associated blocks", func.name)
  }
  for i in 0..func.ty.inputs.len() {
    llvm_values.push(llvm::Value::get_param(llfunc, i as u32))
  }
  let builder = llvm::Builder::new();
  for i in 0..func.blocks.len() {
    llvm_blocks.push(llvm::BasicBlock::append(llfunc, i as u32));
  }

  for (i, block) in func.blocks.iter().enumerate() {
    builder.position_at_end(llvm_blocks[i]);
    build_block(block, &builder, functions, &llvm_blocks, &mut llvm_values);
  }
}

fn build_block<'a>(blk: &Block<'a>, builder: &llvm::Builder,
    functions: &HashMap<&Function<'a>, llvm::Value>,
    blocks: &[llvm::BasicBlock], values: &mut Vec<llvm::Value>) {
  for value in &*blk.block_values.borrow() {
    build_value(value, builder, functions, values);
  }
  build_terminator(blk.terminator.get(), &builder, blocks, values);
}

fn build_value<'a>(value: &Value<'a>, builder: &llvm::Builder,
    functions: &HashMap<&Function<'a>, llvm::Value>,
    values: &mut Vec<llvm::Value>) {
  use core::function::ValueKind;
  let llval = match value.kind {
    ValueKind::ConstInt {
      ty,
      value,
    } => {
      llvm::Value::const_int(llvm::get_int_type(ty.int_size()), value)
    }
    ValueKind::Call {
      function,
      ref parameters
    } => {
      let mut llvm_params = vec![];
      for param in parameters.iter() {
        llvm_params.push(values[param.number as usize]);
      }
      builder.build_call(*functions.get(function).expect("pcb_ice: Blorghle"),
        &llvm_params)
    }
    ValueKind::Mul(lhs, rhs) => {
      builder.build_mul(values[lhs.number as usize],
        values[rhs.number as usize])
    }
    ValueKind::UDiv(lhs, rhs) => {
      builder.build_udiv(values[lhs.number as usize],
        values[rhs.number as usize])
    }
    ValueKind::SDiv(lhs, rhs) => {
      builder.build_sdiv(values[lhs.number as usize],
        values[rhs.number as usize])
    }
    ValueKind::URem(lhs, rhs) => {
      builder.build_urem(values[lhs.number as usize],
        values[rhs.number as usize])
    }
    ValueKind::SRem(lhs, rhs) => {
      builder.build_srem(values[lhs.number as usize],
        values[rhs.number as usize])
    }

    ValueKind::Add(lhs, rhs) => {
      builder.build_add(values[lhs.number as usize],
        values[rhs.number as usize])
    }
    ValueKind::Sub(lhs, rhs) => {
      builder.build_sub(values[lhs.number as usize],
        values[rhs.number as usize])
    }

    ValueKind::Shl(lhs, rhs) => {
      builder.build_shl(values[lhs.number as usize],
        values[rhs.number as usize])
    }
    ValueKind::ZShr(lhs, rhs) => {
      builder.build_ashr(values[lhs.number as usize],
        values[rhs.number as usize])
    }
    ValueKind::SShr(lhs, rhs) => {
      builder.build_lshr(values[lhs.number as usize],
        values[rhs.number as usize])
    }

    ValueKind::And(lhs, rhs) => {
      builder.build_and(values[lhs.number as usize],
        values[rhs.number as usize])
    }
    ValueKind::Xor(lhs, rhs) => {
      builder.build_xor(values[lhs.number as usize],
        values[rhs.number as usize])
    }
    ValueKind::Or(lhs, rhs) => {
      builder.build_or(values[lhs.number as usize],
        values[rhs.number as usize])
    }

    ValueKind::Eq(_, _) => unimplemented!(),
    ValueKind::Neq(_, _) => unimplemented!(),
    ValueKind::Lt(_, _) => unimplemented!(),
    ValueKind::Gt(_, _) => unimplemented!(),
    ValueKind::Lte(_, _) => unimplemented!(),
    ValueKind::Gte(_, _) => unimplemented!(),
    ValueKind::Parameter(_) => panic!("pcb_ice: Parameter should never be \
      built"),
  };
  values.push(llval)
}

fn build_terminator(term: Terminator, builder: &llvm::Builder,
    blocks: &[llvm::BasicBlock], values: &[llvm::Value]) {
  match term {
    Terminator::Branch(b) => {
      builder.build_br(blocks[b.number as usize]);
    },
    Terminator::Return(r) => {
      builder.build_ret(values[r.number as usize]);
    }
    Terminator::None => {
      panic!("pcb_assert: no terminator set")
    }
  }
}
