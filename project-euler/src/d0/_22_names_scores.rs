use std::fs;

fn _solution() -> i32{
    let names = fs::read_to_string("../../resources/d0/_22_names.txt").unwrap();
    let mut names: Vec<&str> = names.split(',').collect();
    names.sort();

    let mut total = 0;
    for (i, name) in names.iter().enumerate() {
        let mut score = 0;
        for c in name.chars() {
            if c.is_alphabetic() {
                score += c as i32 - 'A' as i32 + 1;
            }
        }
        total += score * (i as i32 + 1);
    }

    total
}

#[test]
fn test() {
    println!("{}", _solution());
}
