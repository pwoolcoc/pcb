//use super::Ctxt;
use common::Interner;

pub type TypeContext = Interner<Type>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Type(TypeKind);

impl Type {
  pub(crate) fn variant(&self) -> &TypeKind {
    &self.0
  }
  pub(crate) fn new(inner: TypeKind) -> Type {
    Type(inner)
  }

  pub(crate) fn int_size(&self) -> u32 {
    match self.0 {
      TypeKind::Integer(size) => size,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum TypeKind {
  Integer(u32),
  /*
  Bool,
  Pointer,
  // FnPtr
  Aggregate(Vec<Type<'c>>),
  */
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Function<'a> {
  //input: Box<[Type<'c>]>,
  output: &'a Type,
}

impl<'a> Function<'a> {
  pub fn new(_inputs: Vec<&'a Type>, output: &'a Type) -> Self {
    Function {
      output: output,
    }
  }

  #[inline(always)]
  pub fn output(&self) -> &'a Type {
    self.output
  }
}

mod fmt {
  use std::fmt::{Display, Formatter, Error};
  use super::{Type, TypeKind, Function};
  impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      match self.0 {
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
      write!(f, "() -> {}", self.output)
    }
  }
}

