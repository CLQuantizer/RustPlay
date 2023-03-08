use std::ops::Deref;

struct Thing<T>{
    value: T
}
impl <T> Deref for Thing<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn main() {
    println!("Hello, world!");
    let a = "Ezio".to_string();
    let thing: Thing<String> = Thing {value: a};
    assert_eq!("Ezio", thing.deref());
    assert_eq!("Ezio", *thing);

    let x = &mut Some(ListNode{val: 1, next: None });
    let y = &mut Some(2);
    let z = &mut 3;
    
    *x = Some(ListNode::new(11));
    *y = Some(22);
    *z = 33;
    println!("{}", x.as_ref().unwrap().val);
    println!("{}", y.unwrap());
    println!("{}", z);
    
    let mut m = &mut "ABC".to_string();
    println!("{}", m);
    *m = "CBA".to_string();
    println!("{}", m);
    let mut tmp = "XYZ".to_string();
    m = &mut tmp;
    println!("{}", m);
    *m = "you got an F".to_string(); 
    println!("{}", m);
}
