use num_bigint::BigUint;
use num_traits::One;

fn _solution() -> u32 {
    let num: BigUint = BigUint::one() << 1000; 
    let num_str = num.to_string();
    let sum = num_str.chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();
    sum
}

#[test]
fn test() {
    println!("{}", _solution());
}