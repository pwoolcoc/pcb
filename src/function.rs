use common::Context;
use {ty_, llvm};
use std::fmt::{self, Display, Formatter};
use std::cell::{Cell, RefCell};

pub(crate) type FuncContext<'f, 't> = Context<Function<'f, 't>>;

pub struct Function<'f, 't: 'f> {
  name: String,
  ty: ty_::Function<'t>,
  values: RefCell<Vec<Value<'f, 't>>>, // 'self, 't
  blocks: BlockContext<'f, 't>, // 'self, 't
  llvm: Cell<Option<llvm::Value>>,
}

impl<'f, 't> Function<'f, 't> {
  pub fn new(name: &str, ty: ty_::Function<'t>) -> Self {
    Function {
      name: name.to_owned(),
      ty: ty,
      values: RefCell::new(vec![]),
      llvm: Cell::new(None),
      blocks: BlockContext::new(),
    }
  }

  pub fn add_block(&'f self) -> &'f Block<'f, 't> {
    self.blocks.push(Block::new(self, self.blocks.len() as u32))
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

impl<'f, 't> Display for Function<'f, 't> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    try!(writeln!(f, "define {}{} {{", self.name, self.ty));
    for blk in &self.blocks {
      try!(write!(f, "{}", blk));
    }
    write!(f, "}}")
  }
}

pub(crate) type BlockContext<'f, 't> = Context<Block<'f, 't>>;

#[derive(Copy, Clone)]
pub(crate) enum Terminator<'f, 't: 'f> {
  Branch(&'f Block<'f, 't>),
  // final return in a function
  Return(&'f Value<'f, 't>),
  None,
}

impl<'f, 't> Terminator<'f, 't> {
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

impl<'f, 't> Display for Terminator<'f, 't> {
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
pub struct Block<'f, 't: 'f> {
  number: u32,
  terminator: Cell<Terminator<'f, 't>>,
  block_values: RefCell<Vec<&'f Value<'f, 't>>>,
  llvm: Cell<Option<llvm::BasicBlock>>,
  func: &'f Function<'f, 't>,
}

impl<'f, 't> Block<'f, 't> {
  fn new(function: &'f Function<'f, 't>, num: u32) -> Self {
    Block {
      number: num,
      terminator: Cell::new(Terminator::None),
      block_values: RefCell::new(vec![]),
      llvm: Cell::new(None),
      func: function,
    }
  }

  pub fn set_terminator_branch(&self, b: &'f Block<'f, 't>) {
    assert!(self.func as *const _ == b.func as *const _,
        "pcb_assert: branch is not to a block from the same function");
    self.terminator.set(Terminator::Branch(b));
  }

  pub fn set_terminator_return(&self, v: &'f Value<'f, 't>) {
    assert!(self.func as *const _ == v.func as *const _,
        "pcb_assert: Value is not from the same function as block");
    self.terminator.set(Terminator::Return(v));
  }

  pub fn build_const_int(&self, ty: &'t ty_::Type, value: u64)
      ->  &Value<'f, 't> {
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
      let ret = transmute::<&Value, &Value>(&borrow[borrow.len() - 1]);
      self.block_values.borrow_mut().push(ret);
      ret
    }
  }

  pub fn build_call(&'f self, func: &'f Function<'f, 't>) -> &'f Value<'f, 't> {
    use std::mem::transmute;
    let mut borrow = self.func.values.borrow_mut();
    let len = borrow.len();
    borrow.push(
      Value::new(
        ValueKind::Call(unsafe { transmute::<&Function, &Function>(func) }),
        len as u32,
        self.func,
    ));
    unsafe {
      let ret = transmute::<&Value, &Value>(&borrow[borrow.len() - 1]);
      self.block_values.borrow_mut().push(ret);
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

impl<'f, 't> Display for Block<'f, 't> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    try!(writeln!(f, "bb{}:", self.number));
    for value in &*self.block_values.borrow() {
      try!(writeln!(f, "  %{}: {} = {}", value.number(), value.ty(), value));
    }
    try!(writeln!(f, "  {}", self.terminator.get()));
    Ok(())
  }
}

pub struct Value<'f, 't: 'f> {
  number: u32,
  kind: ValueKind<'f, 't>,
  llvm: Cell<Option<llvm::Value>>,
  func: &'f Function<'f, 't>,
}
impl<'f, 't> Value<'f, 't> {
  pub fn ty(&self) -> &'t ty_::Type {
    match self.kind {
      ValueKind::ConstInt {
        ty,
        ..
      } => ty,
      ValueKind::Call(f) => f.ty.output(),
    }
  }

  fn new(kind: ValueKind<'f, 't>, number: u32, func: &'f Function<'f, 't>)
      -> Self {
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

enum ValueKind<'f, 't: 'f> {
  ConstInt {
    ty: &'t ty_::Type,
    value: u64,
  },
  Call(&'f Function<'f, 't>),
}

impl<'f, 't> Display for Value<'f, 't> {
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
