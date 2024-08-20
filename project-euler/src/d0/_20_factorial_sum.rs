use num_bigint::BigUint;
use num_traits::One;

fn _solution() -> u32 {
    let mut factorial: BigUint = BigUint::one();

    for i in 1..=100 {
        factorial *= BigUint::from(i as u32);
    }

    let mut result = 0;

    for c in factorial.to_string().chars() {
        result += c.to_digit(10).unwrap();
    }

    result
}

#[test]
fn test() {
    println!("{}", _solution());
}