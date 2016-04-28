use std;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Display, Formatter};
use std::cell::{self, RefCell};
use std::collections::HashSet;

use typed_arena::Arena;

#[repr(C)]
pub(crate) struct Ref<T: ?Sized>(*const T);
impl<T: ?Sized> Ref<T> {
  pub unsafe fn from_ref(ref_: &T) -> Ref<T> { Ref(ref_) }
  pub unsafe fn to_ref<'a>(self) -> &'a T { &*self.0 }
  pub fn as_ptr(&self) -> *const T { self.0 }
}
impl<T: ?Sized> Copy for Ref<T> { }
impl<T: ?Sized> Clone for Ref<T> { fn clone(&self) -> Self { *self } }
impl<T: PartialEq> PartialEq for Ref<T> {
  fn eq(&self, rhs: &Self) -> bool {
    unsafe { *self.0 == *rhs.0 }
  }
}
impl<T: Eq> Eq for Ref<T> { }
impl<T: Hash> Hash for Ref<T> {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    unsafe {
      (*self.0).hash(state);
    }
  }
}
impl<T: ?Sized> std::ops::Deref for Ref<T> {
  type Target = T;
  fn deref(&self) -> &T {
    unsafe { &*self.0 }
  }
}
impl<T: ?Sized> Debug for Ref<T> where T: Debug {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    unsafe { (*self.0).fmt(f) }
  }
}
impl<T: ?Sized> Display for Ref<T> where T: Display {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    unsafe { (*self.0).fmt(f) }
  }
}

#[repr(C)]
pub(crate) struct RefMut<T: ?Sized>(*mut T);
impl<T: ?Sized> RefMut<T> {
  pub unsafe fn from_ref(ref_: &mut T) -> RefMut<T> { RefMut(ref_) }
  pub unsafe fn to_ref<'a>(self) -> &'a mut T { &mut *self.0 }
}
impl<T: ?Sized> Copy for RefMut<T> { }
impl<T: ?Sized> Clone for RefMut<T> { fn clone(&self) -> Self { *self } }
impl<T: PartialEq> PartialEq for RefMut<T> {
  fn eq(&self, rhs: &Self) -> bool {
    unsafe { *self.0 == *rhs.0 }
  }
}
impl<T: Eq> Eq for RefMut<T> { }
impl<T: Hash> Hash for RefMut<T> {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    unsafe {
      (*self.0).hash(state);
    }
  }
}

impl<T: ?Sized> std::ops::Deref for RefMut<T> {
  type Target = T;
  fn deref(&self) -> &T {
    unsafe { &*self.0 }
  }
}
impl<T: ?Sized> std::ops::DerefMut for RefMut<T> {
  fn deref_mut(&mut self) -> &mut T {
    unsafe { &mut *self.0 }
  }
}
impl<T: ?Sized> Debug for RefMut<T> where T: Debug {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    unsafe { (*self.0).fmt(f) }
  }
}

pub(crate) struct Context<T> {
  store: Arena<T>,
  vec: RefCell<Vec<*const T>>, // for iterators
}

impl<T> Context<T> {
  pub fn new() -> Self {
    Context {
      store: Arena::new(),
      vec: RefCell::new(vec![])
    }
  }
  pub fn push(&self, variant: T) -> RefMut<T> {
    let id = self.store.alloc(variant);
    self.vec.borrow_mut().push(id);
    unsafe { RefMut::from_ref(id) }
  }
  pub fn len(&self) -> usize {
    self.vec.borrow().len()
  }

  pub fn iter(&self) -> ContextIter<T> {
    ContextIter {
      vec: unsafe {
        ::std::mem::transmute::<cell::Ref<Vec<*const T>>,
                                cell::Ref<Vec<&T>>>(self.vec.borrow())
      },
      idx: 0,
    }
  }
}

impl<'a, T> IntoIterator for &'a Context<T> {
    type Item = &'a T;
    type IntoIter = ContextIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
      self.iter()
    }
}

pub struct ContextIter<'a, T: 'a> {
  vec: cell::Ref<'a, Vec<&'a T>>,
  idx: usize,
}

impl<'a, T> Iterator for ContextIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<&'a T> {
    if self.idx >= self.vec.len() {
      None
    } else {
      let ret = Some(self.vec[self.idx]);
      self.idx += 1;
      ret
    }
  }
}

/*
pub struct InternRef<T>(*const T);
impl<T> ::InternRefExt<T> for InternRef<T> {
  // DO NOT CALL THIS EXCEPT FOR ON AN UNMAKE'D PTR
  unsafe fn make(ptr: *const T) -> InternRef<T> {
    InternRef(ptr)
  }
  fn unmake(self) -> *const T {
    self.0
  }
}
impl<T> Copy for InternRef<T> { }
impl<T> Clone for InternRef<T> { fn clone(&self) -> Self { *self } }
impl<T> PartialEq for InternRef<T> {
  fn eq(&self, rhs: &Self) -> bool {
    self.0 as *const _ == rhs.0 as *const _
  }
}
impl<T> Eq for InternRef<T> { }

impl<T> Hash for InternRef<T> {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    (self.0 as *const _).hash(state);
  }
}

impl<T> Debug for InternRef<T> where T: Debug {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    (self.0).fmt(f)
  }
}

impl<T> Display for InternRef<T> where T: Display {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    (self.0).fmt(f)
  }
}
*/

pub struct Interner<T> {
  store: Arena<T>,
  refs: RefCell<HashSet<HashPtr<T>>>,
}

impl<T: Hash + Eq> Interner<T> {
  pub fn new() -> Self {
    Interner {
      store: Arena::new(),
      refs: RefCell::new(HashSet::new()),
    }
  }

  pub(crate) fn get(&self, variant: T) -> Ref<T> {
    if let Some(id) = self.refs.borrow().get(&HashPtr(&variant)) {
      return Ref(unsafe { &*id.0 });
    }

    let id = self.store.alloc(variant);
    self.refs.borrow_mut().insert(HashPtr(id));
    Ref(id)
  }
}

// needed because *const T does not hash like T does
// UNSAFE TO USE
struct HashPtr<T>(*const T);

impl<T> Copy for HashPtr<T> {}
impl<T> Clone for HashPtr<T> { fn clone(&self) -> Self { *self } }
impl<T: Debug> Debug for HashPtr<T> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    unsafe {
      (*self.0).fmt(f)
    }
  }
}

impl<T: Hash> Hash for HashPtr<T> {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    unsafe {
      (*self.0).hash(state)
    }
  }
}

impl<T: PartialEq> PartialEq for HashPtr<T> {
  fn eq(&self, rhs: &Self) -> bool {
    unsafe {
      (*self.0) == (*rhs.0)
    }
  }
}

impl<T: Eq> Eq for HashPtr<T> { }
