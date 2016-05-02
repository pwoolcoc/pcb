use std::hash::{Hash, Hasher};
use std::cell::{self, RefCell};
use std::collections::HashSet;

use typed_arena::Arena;

pub struct Context<T> {
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
  pub fn push(&self, variant: T) -> &T {
    let id = self.store.alloc(variant);
    self.vec.borrow_mut().push(id);
    id
  }
  pub fn len(&self) -> usize {
    self.vec.borrow().len()
  }

  pub fn iter(&self) -> ContextIter<T> {
    use std::mem::transmute;
    ContextIter {
      vec: unsafe {
        transmute::<cell::Ref<Vec<*const T>>,
                    cell::Ref<Vec<&T>>>(self.vec.borrow())
      },
      idx: 0,
    }
  }

  pub fn get(&self, idx: usize) -> Option<&T> {
    if idx < self.len() {
      Some(unsafe { &*self.vec.borrow()[idx] })
    } else {
      None
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

  pub fn get(&self, variant: T) -> &T {
    if let Some(id) = self.refs.borrow().get(&HashPtr(&variant)) {
      return unsafe { &*id.0 };
    }

    let id = self.store.alloc(variant);
    self.refs.borrow_mut().insert(HashPtr(id));
    id
  }
}

// needed because *const T does not hash like T does
// UNSAFE TO USE
struct HashPtr<T>(*const T);

impl<T> Copy for HashPtr<T> {}
impl<T> Clone for HashPtr<T> { fn clone(&self) -> Self { *self } }

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
