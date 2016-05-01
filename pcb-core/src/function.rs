use common::Context;
use {ty, llvm};
use std::fmt::{self, Display, Formatter};
use std::cell::{Cell, RefCell};

pub type FuncContext<'c> = Context<Function<'c>>;

pub struct Function<'c> {
  pub name: String,
  pub ty: ty::Function<'c>,
  pub blocks: BlockContext<'c>,
  pub values: ValueContext<'c>,
  pub llvm: Cell<Option<llvm::Value>>,
}

impl<'c> Function<'c> {
  pub fn new(name: &str, ty: ty::Function<'c>) -> Self {
    Function {
      name: name.to_owned(),
      ty: ty,
      values: ValueContext::new(),
      llvm: Cell::new(None),
      blocks: BlockContext::new(),
    }
  }

  pub fn add_block(&'c self) -> &'c Block<'c> {
    self.blocks.push(
      Block {
        number: self.blocks.len() as u32,
        terminator: Cell::new(Terminator::None),
        block_values: RefCell::new(vec![]),
        llvm: Cell::new(None),
        func: self,
      })
  }

  pub fn build(&self) {
    let llfunc = self.llvm.get().expect(
      &format!("llfunc was never set for {}", self.name));
    if self.blocks.iter().next().is_none() {
      panic!("pcb_assert: function {} has no associated blocks", self.name)
    }
    let builder = llvm::Builder::new();
    for (i, block) in self.blocks.iter().enumerate() {
      block.llvm.set(Some(llvm::BasicBlock::append(llfunc, i as u32)));
    }

    for block in &self.blocks {
      builder.position_at_end(block.llvm.get().unwrap());
      block.to_llvm(&builder);
    }
  }

  #[inline(always)]
  pub fn name(&self) -> &str {
    &self.name
  }

  #[inline(always)]
  pub fn ty(&self) -> &ty::Function<'c> {
    &self.ty
  }
}

impl<'c> Display for Function<'c> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    try!(writeln!(f, "define {}{} {{", self.name, self.ty));
    for blk in &self.blocks {
      try!(write!(f, "{}", blk));
    }
    write!(f, "}}")
  }
}

pub type BlockContext<'c> = Context<Block<'c>>;

#[derive(Copy, Clone)]
pub enum Terminator<'c> {
  Branch(&'c Block<'c>),
  // final return in a function
  Return(&'c Value<'c>),
  None,
}

impl<'c> Terminator<'c> {
  fn to_llvm(&self, builder: &llvm::Builder) {
    match *self {
      Terminator::Branch(b) => {
        builder.build_br(b.llvm.get().expect("pcb_ice: All blocks should \
          have associated basic blocks by now"));
      },
      Terminator::Return(r) => {
        builder.build_ret(r.llvm.get().expect("pcb_ice: Value does not \
            have an associated llvm value"));
      }
      Terminator::None => {
        panic!("pcb_assert: no terminator set")
      }
    }
  }

  pub fn is_none(self) -> bool {
    if let Terminator::None = self {
      true
    } else {
      false
    }
  }
}

impl<'c> Display for Terminator<'c> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match *self {
      Terminator::Branch(b) => {
        write!(f, "branch bb{}", b.number)
      },
      Terminator::Return(r) => {
        write!(f, "return %{}", r.number)
      }
      Terminator::None => { Ok(()) }
    }
  }
}

// TODO(ubsan): don't allow terminators to be re-set, and stop allowing stuff to
// build after setting terminator
// .build_return, .build_branch, etc.
pub struct Block<'c> {
  pub number: u32,
  pub terminator: Cell<Terminator<'c>>,
  pub block_values: RefCell<Vec<&'c Value<'c>>>,
  pub llvm: Cell<Option<llvm::BasicBlock>>,
  pub func: &'c Function<'c>,
}

impl<'c> Block<'c> {
  pub fn add_value(&'c self, kind: ValueKind<'c>) -> &'c Value<'c> {
    let ret = self.func.values.push(
      Value {
        number: self.func.values.len() as u32,
        kind: kind,
        llvm: Cell::new(None),
        func: &self.func,
      });
    self.block_values.borrow_mut().push(ret);
    ret
  }

  fn to_llvm(&self, builder: &llvm::Builder) {
    for value in &*self.block_values.borrow() {
      value.to_llvm(builder);
    }
    self.terminator.get().to_llvm(builder);
  }
}

impl<'c> Display for Block<'c> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    try!(writeln!(f, "bb{}:", self.number));
    for value in &*self.block_values.borrow() {
      try!(writeln!(f, "  %{}: {} = {}", value.number, value.ty(), value));
    }
    try!(writeln!(f, "  {}", self.terminator.get()));
    Ok(())
  }
}

pub type ValueContext<'c> = Context<Value<'c>>;

pub struct Value<'c> {
  pub number: u32,
  pub kind: ValueKind<'c>,
  pub llvm: Cell<Option<llvm::Value>>,
  pub func: &'c Function<'c>,
}
impl<'c> Value<'c> {
  pub fn ty(&self) -> &'c ty::TypeKind {
    match self.kind {
      ValueKind::ConstInt {
        ty,
        ..
      } => ty,
      ValueKind::Call(f) => f.ty.output,
    }
  }

  fn to_llvm(&self, builder: &llvm::Builder) {
    self.llvm.set(Some(match self.kind {
      ValueKind::ConstInt {
        ty,
        value,
      } => {
        llvm::Value::const_int(llvm::get_int_type(ty.int_size()), value)
      }
      ValueKind::Call(f) => {
        builder.build_call(f.llvm.get().expect("pcb_ice: all pcb IR functions \
          should have associated llvm IR functions by now"), &[])
      }
    }))
  }
}

pub enum ValueKind<'c> {
  ConstInt {
    ty: &'c ty::TypeKind,
    value: u64,
  },
  Call(&'c Function<'c>),
}

impl<'c> Display for Value<'c> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match self.kind {
      ValueKind::ConstInt {
        value,
        ..
      } => {
        try!(write!(f, "{}", value));
      }
      ValueKind::Call(ref func) => {
        try!(write!(f, "call {}()", func.name()));
      }
    }
    Ok(())
  }
}
