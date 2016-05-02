use common::Context;
use ty;
use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::cell::{Cell, RefCell};

pub type FuncContext<'c> = Context<Function<'c>>;

pub struct Function<'c> {
  pub name: String,
  pub ty: ty::Function<'c>,
  pub blocks: BlockContext<'c>,
  pub values: ValueContext<'c>,
}

impl<'c> Function<'c> {
  pub fn new(name: &str, ty: ty::Function<'c>) -> Self {
    Function {
      name: name.to_owned(),
      ty: ty,
      values: ValueContext::new(),
      blocks: BlockContext::new(),
    }
  }

  pub fn add_block(&'c self) -> &'c Block<'c> {
    self.blocks.push(
      Block {
        number: self.blocks.len() as u32,
        terminator: Cell::new(Terminator::None),
        block_values: RefCell::new(vec![]),
        func: self,
      })
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

impl<'c> PartialEq for Function<'c> {
  fn eq(&self, rhs: &Self) -> bool {
    self.name == rhs.name
  }
}
impl<'c> Eq for Function<'c> { }
impl<'c> Hash for Function<'c> {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    self.name.hash(state)
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
  pub func: &'c Function<'c>,
}

impl<'c> Block<'c> {
  pub fn add_value(&'c self, kind: ValueKind<'c>) -> &'c Value<'c> {
    let ret = self.func.values.push(
      Value {
        number: self.func.values.len() as u32,
        kind: kind,
        func: &self.func,
      });
    self.block_values.borrow_mut().push(ret);
    ret
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
        try!(write!(f, "call {}()", func.name));
      }
    }
    Ok(())
  }
}
