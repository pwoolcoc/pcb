//use super::Ctxt;
use common::Interner;

pub type TypeContext = Interner<Type>;

impl Type {
  pub fn int_size(&self) -> u32 {
    match *self {
      Type::Integer(size) => size,
      ref ty => panic!("pcb_ice: tried to take int_size of a non_int type: {}",
                       ty)
    }
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
  Integer(u32),
  Bool,
  /*
  Void,
  Pointer,
  // FnPtr
  Aggregate(Vec<Type<'c>>),
  */
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Function<'t> {
  pub inputs: Box<[&'t Type]>,
  pub output: &'t Type,
}

mod fmt {
  use std::fmt::{Display, Formatter, Error};
  use super::{Type, Function};
  impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      match *self {
        Type::Integer(n) => write!(f, "i{}", n),
        Type::Bool => write!(f, "bool"),
        /*
        Type::Pointer => write!(f, "ptr"),
        Type::Aggregate(ref v) => {
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

