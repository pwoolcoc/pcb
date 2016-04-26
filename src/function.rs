use common::{Context, Ref};
use {ty_, llvm};
use std::fmt::{self, Display, Formatter};
use std::cell::{Cell, RefCell};

pub(crate) type FuncContext<'a> = Context<Function<'a>>;

pub struct Function<'a> {
  name: String,
  ty: ty_::Function<'a>,
  blocks: BlockContext, // 'self
  llvm: Cell<Option<llvm::Value>>,
}

impl<'a> Function<'a> {
  pub fn new(name: &str, ty: ty_::Function<'a>) -> Self {
    Function {
      name: name.to_owned(),
      ty: ty,
      llvm: Cell::new(None),
      blocks: BlockContext::new(),
    }
  }

  pub fn add_block(&self) -> &Block {
    unsafe {
      self.blocks.push(Block::new(self, self.blocks.len() as u32)).to_ref()
    }
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
      block.terminator.get() .to_llvm(&builder);
    }
  }

  #[inline(always)]
  pub fn name(&self) -> &str {
    &self.name
  }

  #[inline(always)]
  pub fn ty(&self) -> &ty_::Function {
    &self.ty
  }

  #[inline(always)]
  pub(crate) fn llvm(&self) -> &Cell<Option<llvm::Value>> {
    &self.llvm
  }
}

impl<'a> Display for Function<'a> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    try!(writeln!(f, "fn {}{} {{", self.name, self.ty));
    for blk in &self.blocks {
      try!(write!(f, "{}", blk));
    }
    Ok(())
  }
}

pub(crate) type BlockContext = Context<Block>;

#[derive(Copy, Clone)]
pub(crate) enum Terminator {
  Branch(Ref<Block>),
  // final return in a function
  Return(Ref<Value>),
  None,
}

impl Terminator {
  fn to_llvm(&self, builder: &llvm::Builder) {
    match *self {
      Terminator::Branch(b) => {
        builder.build_br(b.llvm.get().expect("pcb_assert: All blocks should \
          have associated basic blocks by now"));
      },
      Terminator::Return(r) => {
        if r.is_void() {
          builder.build_void_ret();
        } else {
          builder.build_ret(r.to_llvm(builder))
        }
      }
      Terminator::None => {
        panic!("pcb_assert: no terminator set")
      }
    }
  }
}

impl Display for Terminator {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match *self {
      Terminator::Branch(b) => {
        write!(f, "branch bb{}", b.number)
      },
      Terminator::Return(r) => {
        write!(f, "return {}", r)
      }
      Terminator::None => { Ok(()) }
    }
  }
}

pub struct Block {
  number: u32,
  terminator: Cell<Terminator>,
  llvm: Cell<Option<llvm::BasicBlock>>,
  values: RefCell<Vec<Value>>,
  func: *const (),
}

impl Block {
  fn new(function: &Function, num: u32) -> Block {
    Block {
      number: num,
      terminator: Cell::new(Terminator::None),
      llvm: Cell::new(None),
      values: RefCell::new(vec![]),
      func: function as *const _ as *const (),
    }
  }

  pub fn set_terminator_branch<'a>(&'a self, b: &'a Block) {
    assert!(self.func == b.func, "pcb_assert: branch is not to a block from \
      the same function");
    unsafe {
      self.terminator.set(Terminator::Branch(Ref::from_ref(b)));
    }
  }

  pub fn set_terminator_return(&self, v: &Value) {
    unsafe { self.terminator.set(Terminator::Return(Ref::from_ref(v))); }
  }

  pub fn build_const_int(&self, size: u32, value: u64) ->  &Value {
    use std::mem::transmute;
    let mut borrow = self.values.borrow_mut();
    borrow.push(Value(ValueKind::ConstInt { size: size, value: value, }));
    unsafe { transmute(&borrow[borrow.len() - 1]) }
  }

  pub fn build_call<'a>(&'a self, func: &'a Function<'a>) -> &'a Value {
    use std::mem::transmute;
    let mut borrow = self.values.borrow_mut();
    borrow.push(Value(ValueKind::Call(
      unsafe { Ref::from_ref(transmute(func)) }
    )));
    unsafe { transmute(&borrow[borrow.len() - 1]) }
  }
}

impl Display for Block {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    try!(writeln!(f, "  bb{}: {{", self.number));
    try!(writeln!(f, "    {}", self.terminator.get()));
    try!(writeln!(f, "  }}"));
    Ok(())
  }
}

pub struct Value(ValueKind);

enum ValueKind {
  ConstInt {
    size: u32,
    value: u64,
  },
  Call(Ref<Function<'static>>),
}

impl Value {
  fn to_llvm(&self, builder: &llvm::Builder) -> llvm::Value {
    match self.0 {
      ValueKind::ConstInt {
        size,
        value,
      } => {
        llvm::Value::const_int(llvm::get_int_type(size), value)
      }
      ValueKind::Call(f) => {
        builder.build_call(f.llvm.get().expect("pcb_ice: all pcb IR functions \
          should have associated llvm IR functions by now"), &[])
      }
    }
  }

  fn is_void(&self) -> bool {
    match self.0 {
      ValueKind::ConstInt {
        ..
      } => false,
      ValueKind::Call(_f) => {
        false
      }
    }
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match self.0 {
      ValueKind::ConstInt {
        size,
        value,
      } => {
        try!(write!(f, "{}: i{}", value, size));
      }
      ValueKind::Call(ref func) => {
        try!(write!(f, "call {}()", func.name()));
      }
    }
    Ok(())
  }
}
