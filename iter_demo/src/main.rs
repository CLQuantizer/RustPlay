fn comparison() {
    let needle = 0xcb;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];
    for item in haystack.into_iter() {
        // we do not need deref here
        if item == needle {
            println!("{}", item);
        }
    }
}

fn time_demo() {
    use std::time::{Duration, Instant};
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while Instant::now() - start < time_limit {
        count += 1;
    }
    println!("{}", count);
}

fn main() {
    time_demo();
    comparison();
    let needle = 0xcb;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];
    for item in haystack.iter() {
        if *item == needle {
            println!("{}", item);
        }
    }
}