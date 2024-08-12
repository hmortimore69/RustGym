fn _solution() -> i32 {
    let sum_of_squares = (1..=100).map(|x| x * x).sum::<i32>();
    let square_of_sum = (1..=100).sum::<i32>().pow(2);

    square_of_sum - sum_of_squares
}

#[test]
fn test() {
    println!("Largest palindrome: {}", _solution());
}