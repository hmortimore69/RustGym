fn _collatz_length(mut n: u64) -> u64 {
    let mut length = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }
    length
}

fn _solution() -> u64 {
    let mut max_length = 0;
    let mut starting_number = 0;

    for i in 1..1_000_000 {
        let length = _collatz_length(i);
        if length > max_length {
            max_length = length;
            starting_number = i;
        }
    }

    starting_number
}

#[test]
fn test() {
    println!("{}", _solution());
}
