fn _solution() -> i32 {
    let n = 20;
    let check_list = [11, 13, 14, 16, 17, 18, 19, 20];

    for i in (n..i32::MAX).step_by(n as usize) {
        if check_list.iter().all(|&x| i % x == 0) {
            return i;
        }
    }

    0
}

#[test]
fn test() {
    println!("Largest palindrome: {}", _solution());
}
