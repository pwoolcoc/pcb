use common::Context;
use ty;
use std::fmt::{self, Debug, Display, Formatter};
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
      try!(write!(f, "{:?}", blk));
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
        write!(f, "branch {}", b)
      },
      Terminator::Return(r) => {
        write!(f, "return {}", r)
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

impl<'c> Debug for Block<'c> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    try!(writeln!(f, "{}:", self));
    for value in &*self.block_values.borrow() {
      try!(writeln!(f, "  {}: {} = {:?}", value, value.ty(), value));
    }
    try!(writeln!(f, "  {}", self.terminator.get()));
    Ok(())
  }
}

impl<'c> Display for Block<'c> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "bb{}", self.number)
  }
}

pub type ValueContext<'c> = Context<Value<'c>>;

pub struct Value<'c> {
  pub number: u32,
  pub kind: ValueKind<'c>,
  pub func: &'c Function<'c>,
}
impl<'c> Value<'c> {
  pub fn ty(&self) -> &'c ty::Type {
    match self.kind {
      ValueKind::ConstInt {
        ty,
        ..
      } => ty,
      ValueKind::Call {
        function,
        ..
      } => function.ty.output,
      ValueKind::Parameter(ty) => ty,
      ValueKind::Add(lhs, _) => lhs.ty(),
    }
  }

}

pub enum ValueKind<'c> {
  ConstInt {
    ty: &'c ty::Type,
    value: u64,
  },
  Call {
    function: &'c Function<'c>,
    parameters: Box<[&'c Value<'c>]>
  },
  Add(&'c Value<'c>, &'c Value<'c>),
  Parameter(&'c ty::Type),
}

impl<'c> Debug for Value<'c> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match self.kind {
      ValueKind::ConstInt {
        value,
        ..
      } => {
        try!(write!(f, "{}", value));
      }
      ValueKind::Call {
        function,
        ref parameters
      } => {
        try!(write!(f, "call {}(", function.name));
        if !parameters.is_empty() {
          for i in 0..parameters.len() - 1 {
            try!(write!(f, "%{}, ", i));
          }
          try!(write!(f, "%{}", parameters.len() - 1));
        }
        try!(write!(f, ")"));
      }
      ValueKind::Parameter(_) => panic!("pcb_ice: Parameters should not be \
        displayed"),
      ValueKind::Add(lhs, rhs) =>
        try!(write!(f, "add {} {}", lhs, rhs))
    }
    Ok(())
  }
}

impl<'c> Display for Value<'c> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "%{}", self.number)
  }
}
