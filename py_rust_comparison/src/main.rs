struct Dog {
    id: u8
}

fn main() {
    let mut l = vec![Dog{id: 1},Dog{id: 2},Dog{id: 3}];
    let dog3 = l.get(2).unwrap();
    l.pop(); // 👈 I get error warning here!
    println!("{}",dog3.id);
}
