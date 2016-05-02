//use super::Ctxt;
use common::Interner;

pub type TypeContext = Interner<TypeKind>;

impl TypeKind {
  pub fn int_size(&self) -> u32 {
    match *self {
      TypeKind::Integer(size) => size,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TypeKind {
  Integer(u32),
  /*
  Void,
  Bool,
  Pointer,
  // FnPtr
  Aggregate(Vec<Type<'c>>),
  */
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Function<'t> {
  pub inputs: Box<[&'t TypeKind]>,
  pub output: &'t TypeKind,
}

mod fmt {
  use std::fmt::{Display, Formatter, Error};
  use super::{TypeKind, Function};
  impl Display for TypeKind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      match *self {
        TypeKind::Integer(n) => write!(f, "i{}", n),
        /*
        TypeVariant::Bool => write!(f, "bool"),
        TypeVariant::Pointer => write!(f, "ptr"),
        TypeVariant::Aggregate(ref v) => {
          try!(write!(f, "("));
          if v.is_empty() {
            write!(f, ")")
          } else {
            for el in &v[..v.len() - 1] {
              try!(write!(f, "{}, ", el));
            }
            write!(f, "{})", &v[v.len() - 1])
          }
        }
        */
      }
    }
  }

  impl<'a> Display for Function<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      try!(write!(f, "("));
      if !self.inputs.is_empty() {
        for input in &self.inputs[..self.inputs.len() - 1] {
          try!(write!(f, "{}, ", input));
        }
        try!(write!(f, "{}", self.inputs[self.inputs.len() - 1]));
      }
      write!(f, ") -> {}", self.output)
    }
  }
}

