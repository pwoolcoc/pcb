extern crate pcb_core as core;

use core::pcb::Ctxt;
use core::backend::Backend;
use core::function::{Block, Function, Value, Terminator};

use std::collections::HashMap;

mod llvm;

pub struct Llvm;

impl Backend for Llvm {
  fn build_and_write(ctxt: Ctxt, output: &str, print_llvm_ir: bool) {
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

    match target_machine.emit_to_file(&module, output) {
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
    ValueKind::Call(f) => {
      builder.build_call(*functions.get(f).expect("pcb_ice: Blorghle"), &[])
    }
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
