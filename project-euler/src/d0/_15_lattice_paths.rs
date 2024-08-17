use num_bigint::BigUint;

fn _solution() -> BigUint {
    let x = 20;
    let y = 20;

    _factorial(x+y) / (_factorial(x) * _factorial(x))
}

fn _factorial(num: u128) -> BigUint {
    let result: BigUint = (1..=num).product();

    BigUint::from(result)
}

#[test]
fn test() {
    println!("{}", _solution());
}