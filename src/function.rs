use common::{Context, Ref};
use {ty_, llvm};
use std::fmt::{self, Display, Formatter};
use std::cell::{Cell, RefCell};

pub(crate) type FuncContext<'t> = Context<Function<'t>>;

pub struct Function<'t> {
  name: String,
  ty: ty_::Function<'t>,
  values: RefCell<Vec<Value<'t>>>,
  blocks: BlockContext<'static, 't>, // 'self, 't
  llvm: Cell<Option<llvm::Value>>,
}

impl<'t> Function<'t> {
  pub fn new(name: &str, ty: ty_::Function<'t>) -> Self {
    Function {
      name: name.to_owned(),
      ty: ty,
      values: RefCell::new(vec![]),
      llvm: Cell::new(None),
      blocks: BlockContext::new(),
    }
  }

  pub fn add_block(&self) -> &Block<'t> {
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
      block.to_llvm(&builder);
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
    write!(f, "}}")
  }
}

pub(crate) type BlockContext<'f, 't> = Context<Block<'t>>;

#[derive(Copy, Clone)]
pub(crate) enum Terminator<'t> {
  Branch(Ref<Block<'t>>),
  // final return in a function
  Return(Ref<Value<'t>>),
  None,
}

impl<'t> Terminator<'t> {
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
}

impl<'t> Display for Terminator<'t> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match *self {
      Terminator::Branch(b) => {
        write!(f, "branch bb{}", b.number())
      },
      Terminator::Return(r) => {
        write!(f, "return %{}", r.number())
      }
      Terminator::None => { Ok(()) }
    }
  }
}

// TODO(ubsan): don't allow terminators to be re-set, and stop allowing stuff to
// build after setting terminator
// .build_return, .build_branch, etc.
pub struct Block<'t> {
  number: u32,
  terminator: Cell<Terminator<'t>>, // 'self, 't
  block_values: RefCell<Vec<Ref<Value<'t>>>>, // 'func
  llvm: Cell<Option<llvm::BasicBlock>>,
  func: Ref<Function<'t>>,
}

impl<'t> Block<'t> {
  fn new(function: &Function<'t>, num: u32) -> Self {
    Block {
      number: num,
      terminator: Cell::new(Terminator::None),
      block_values: RefCell::new(vec![]),
      llvm: Cell::new(None),
      func: unsafe { Ref::from_ref(function) },
    }
  }

  pub fn set_terminator_branch(&self, b: &Block<'t>) {
    assert!(self.func.as_ptr() == b.func.as_ptr(), "pcb_assert: branch is not \
        to a block from the same function");
    unsafe {
      self.terminator.set(Terminator::Branch(Ref::from_ref(b)));
    }
  }

  pub fn set_terminator_return(&self, v: &Value<'t>) {
    assert!(self.func.as_ptr() == v.func.as_ptr(), "pcb_assert: Value is not \
        from the same function as block");
    unsafe { self.terminator.set(Terminator::Return(Ref::from_ref(v))); }
  }

  pub fn build_const_int(&self, ty: &'t ty_::Type, value: u64) ->  &Value<'t> {
    use std::mem::transmute;
    let mut borrow = self.func.values.borrow_mut();
    let len = borrow.len();
    borrow.push(
      Value::new(ValueKind::ConstInt {
        ty: ty,
        value: value
      },
      len as u32,
      self.func,
    ));
    unsafe {
      let ret = transmute(&borrow[borrow.len() - 1]);
      self.block_values.borrow_mut().push(Ref::from_ref(ret));
      ret
    }
  }

  pub fn build_call<'a>(&'a self, func: &'a Function<'t>) -> &'a Value<'t> {
    use std::mem::transmute;
    let mut borrow = self.func.values.borrow_mut();
    let len = borrow.len();
    borrow.push(
      Value::new(ValueKind::Call(
        unsafe { Ref::from_ref(transmute(func)) }),
        len as u32,
        self.func
    ));
    unsafe {
      let ret = transmute(&borrow[borrow.len() - 1]);
      self.block_values.borrow_mut().push(Ref::from_ref(ret));
      ret
    }
  }

  fn number(&self) -> u32 { self.number }

  fn to_llvm(&self, builder: &llvm::Builder) {
    for value in &*self.block_values.borrow() {
      value.to_llvm(builder);
    }
    self.terminator.get().to_llvm(builder);
  }
}

impl<'t> Display for Block<'t> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    try!(writeln!(f, "  bb{}: {{", self.number));
    for value in &*self.block_values.borrow() {
      try!(writeln!(f, "    %{}: {} = {}", value.number(), value.ty(), value));
    }
    try!(writeln!(f, "    {}", self.terminator.get()));
    try!(writeln!(f, "  }}"));
    Ok(())
  }
}

pub struct Value<'t> {
  number: u32,
  kind: ValueKind<'t>,
  llvm: Cell<Option<llvm::Value>>,
  func: Ref<Function<'t>>,
}
impl<'t> Value<'t> {
  pub fn ty(&self) -> &'t ty_::Type {
    match self.kind {
      ValueKind::ConstInt {
        ty,
        ..
      } => ty,
      ValueKind::Call(f) => f.ty.output(),
    }
  }

  fn new(kind: ValueKind<'t>, number: u32, func: Ref<Function<'t>>) -> Self {
    Value {
      number: number,
      kind: kind,
      llvm: Cell::new(None),
      func: func,
    }
  }
  fn number(&self) -> u32 { self.number }

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

enum ValueKind<'t> {
  ConstInt {
    ty: &'t ty_::Type,
    value: u64,
  },
  Call(Ref<Function<'t>>),
}

impl<'t> Display for Value<'t> {
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
