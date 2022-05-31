use std::{
  rc::Rc,
  cell::{
    RefCell,
    Ref,
    RefMut
  }
};

pub struct RcCell<A> {
  pub value: Rc<RefCell<A>>
}

impl<A> RcCell<A> {
  pub fn new(value: A) -> RcCell<A> {
    let value = RefCell::new(value);
    let value = Rc::new(value);
    RcCell {
      value
    }
  }

  pub fn clone(&self) -> RcCell<A> {
    let value = Rc::clone(&self.value);
    RcCell {
      value
    }
  }

  pub fn borrow(&self) -> Ref<A> {
    self.value.borrow()
  }

  pub fn borrow_mut(&self) -> RefMut<A> {
    self.value.borrow_mut()
  }

  pub fn update(&self, u: fn(RefMut<A>)) {
    u(self.borrow_mut())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, PartialEq)]
  struct Data<A> {
    value: A
  }

  impl<A> Data<A> {
    fn new(value: A) -> Data<A> {
      Data {
        value,
      }
    }
  }

  #[test]
  fn multiple_readers() {
    let data: Data<i32> = Data::new(1);
    let data = RcCell::new(data);
  
    let clone1 = data.clone();
    let clone2 = data.clone();

    assert_eq!(clone1.borrow().value, 1);
    assert_eq!(*clone1.borrow(), *clone2.borrow());
  }

  #[test]
  fn multiple_writers() {
    let data: Data<i32> = Data::new(1);
    let data = RcCell::new(data);
  
    let clone1 = data.clone();
    clone1.update(|mut d| d.value += 1);
    let clone2 = data.clone();
    clone2.update(|mut d| d.value *= 3);

    assert_eq!(data.borrow().value, 6);
    assert_eq!(*clone1.borrow(), *clone2.borrow());
  }

  #[test]
  fn example() {
    #[derive(Debug, PartialEq)]
    struct Data<A> {
      value: A
    }
  
    impl<A> Data<A> {
      fn new(value: A) -> Data<A> {
        Data {
          value,
        }
      }
    }

    let data: Data<i32> = Data::new(1);

    let counter: RcCell<Data<i32>> = RcCell::new(data);
    let counter_a: RcCell<Data<i32>> = counter.clone();
    let counter_b: RcCell<Data<i32>> = counter.clone();

    counter_a.update(|mut v| v.value += 1);
    counter_b.borrow_mut().value *= 3;

    assert_eq!(*counter_a.borrow(), *counter_b.borrow());
  }
}
