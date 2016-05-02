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
      ValueKind::Mul(lhs, _) => lhs.ty(),
      ValueKind::UDiv(lhs, _) => lhs.ty(),
      ValueKind::SDiv(lhs, _) => lhs.ty(),
      ValueKind::URem(lhs, _) => lhs.ty(),
      ValueKind::SRem(lhs, _) => lhs.ty(),

      ValueKind::Add(lhs, _) => lhs.ty(),
      ValueKind::Sub(lhs, _) => lhs.ty(),

      ValueKind::Shl(lhs, _) => lhs.ty(),
      ValueKind::ZShr(lhs, _) => lhs.ty(),
      ValueKind::SShr(lhs, _) => lhs.ty(),

      ValueKind::And(lhs, _) => lhs.ty(),
      ValueKind::Xor(lhs, _) => lhs.ty(),
      ValueKind::Or(lhs, _) => lhs.ty(),

      ValueKind::Eq(_, _) => unimplemented!(),
      ValueKind::Neq(_, _) => unimplemented!(),
      ValueKind::Lt(_, _) => unimplemented!(),
      ValueKind::Gt(_, _) => unimplemented!(),
      ValueKind::Lte(_, _) => unimplemented!(),
      ValueKind::Gte(_, _) => unimplemented!(),
      ValueKind::Parameter(ty) => ty,
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

  // -- binops --
  Mul(&'c Value<'c>, &'c Value<'c>),
  UDiv(&'c Value<'c>, &'c Value<'c>),
  SDiv(&'c Value<'c>, &'c Value<'c>),
  URem(&'c Value<'c>, &'c Value<'c>),
  SRem(&'c Value<'c>, &'c Value<'c>),

  Add(&'c Value<'c>, &'c Value<'c>),
  Sub(&'c Value<'c>, &'c Value<'c>),

  Shl(&'c Value<'c>, &'c Value<'c>),
  ZShr(&'c Value<'c>, &'c Value<'c>), // zero-extend
  SShr(&'c Value<'c>, &'c Value<'c>), // sign-extend

  And(&'c Value<'c>, &'c Value<'c>),
  Xor(&'c Value<'c>, &'c Value<'c>),
  Or(&'c Value<'c>, &'c Value<'c>),

  Eq(&'c Value<'c>, &'c Value<'c>),
  Neq(&'c Value<'c>, &'c Value<'c>),
  Lt(&'c Value<'c>, &'c Value<'c>),
  Gt(&'c Value<'c>, &'c Value<'c>),
  Lte(&'c Value<'c>, &'c Value<'c>),
  Gte(&'c Value<'c>, &'c Value<'c>),

  // parameter (this *may not* be built; it's simply a placeholder)
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
      ValueKind::Mul(lhs, rhs) => try!(write!(f, "mul {} {}", lhs, rhs)),
      ValueKind::UDiv(lhs, rhs) => try!(write!(f, "udiv {} {}", lhs, rhs)),
      ValueKind::SDiv(lhs, rhs) => try!(write!(f, "sdiv {} {}", lhs, rhs)),
      ValueKind::URem(lhs, rhs) => try!(write!(f, "urem {} {}", lhs, rhs)),
      ValueKind::SRem(lhs, rhs) => try!(write!(f, "srem {} {}", lhs, rhs)),

      ValueKind::Add(lhs, rhs) => try!(write!(f, "add {} {}", lhs, rhs)),
      ValueKind::Sub(lhs, rhs) => try!(write!(f, "sub {} {}", lhs, rhs)),

      ValueKind::Shl(lhs, rhs) => try!(write!(f, "shl {} {}", lhs, rhs)),
      ValueKind::ZShr(lhs, rhs) => try!(write!(f, "zshr {} {}", lhs, rhs)), // zero-extend
      ValueKind::SShr(lhs, rhs) => try!(write!(f, "sshr {} {}", lhs, rhs)), // sign-extend

      ValueKind::And(lhs, rhs) => try!(write!(f, "and {} {}", lhs, rhs)),
      ValueKind::Xor(lhs, rhs) => try!(write!(f, "xor {} {}", lhs, rhs)),
      ValueKind::Or(lhs, rhs) => try!(write!(f, "or {} {}", lhs, rhs)),

      ValueKind::Eq(lhs, rhs) => try!(write!(f, "eq {} {}", lhs, rhs)),
      ValueKind::Neq(lhs, rhs) => try!(write!(f, "neq {} {}", lhs, rhs)),
      ValueKind::Lt(lhs, rhs) => try!(write!(f, "lt {} {}", lhs, rhs)),
      ValueKind::Gt(lhs, rhs) => try!(write!(f, "gt {} {}", lhs, rhs)),
      ValueKind::Lte(lhs, rhs) => try!(write!(f, "lte {} {}", lhs, rhs)),
      ValueKind::Gte(lhs, rhs) => try!(write!(f, "gte {} {}", lhs, rhs)),

      ValueKind::Parameter(_) => panic!("pcb_ice: Parameters should not be \
        displayed"),
    }
    Ok(())
  }
}

impl<'c> Display for Value<'c> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "%{}", self.number)
  }
}
