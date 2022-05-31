# rcrefcell
Simple wrapper for Rc<RefCell<A<A>>>

## Example
```rs
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
```
