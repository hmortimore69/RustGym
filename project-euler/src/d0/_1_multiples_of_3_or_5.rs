fn _solution() -> i32 {
    let mut sum = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    sum
}

#[test]
fn test() {
    println!("{}", _solution());
}
