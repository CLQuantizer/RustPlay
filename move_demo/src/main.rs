fn main() {
    let x = vec![1, 2, 3];
    let y = move_vec(x);
    println!("y: {:?}", y); // y: [1, 2, 3]
    
    // Here, into_iter method is used to move the ownership of the vector 
    // to the iterator and then map method is used to apply a closure 
    // to each element of the iterator, creating a new iterator. 
    // The collect method is then used to collect the iterator into a new vector.

    let u = vec![1, 2, 3];
    let new_u = u.iter().map(|x| x * 2).collect::<Vec<_>>();
    println!("new_u: {:?}", new_u);

    play_with_iter_and_into_iter();

}

fn move_vec(v: Vec<i32>) -> Vec<i32> {
    let result = v;
    result
}

fn play_with_iter_and_into_iter(){
    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));

    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
}