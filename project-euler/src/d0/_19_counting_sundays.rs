fn _solution() -> i32 {
    let mut n = 0;
    let mut dow = 2;
    let mut months = [31, 0, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    for y in 1901..=2000 {
        months[1] = if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
            29
        } else {
            28
        };

        for &month in months.iter() {
            if dow % 7 == 0 {
                n += 1;
            }
            dow = (dow + month) % 7;
        }
    }

    n
}

fn _dow(year: i32, month: usize, day: f32) -> i32 {
    let zero_based_century = year / 100;
    let zero_based_year = year % 100;

    let month = [13, 14, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12][month - 1];

    ((day + (13 * (month + 1) / 5) as f32).floor() as i32
        + zero_based_year
        + (zero_based_year / 4) as i32
        + (zero_based_century / 4) as i32
        + 5 * zero_based_century)
        % 7
}

#[test]
fn test() {
    println!("{}", _solution());
}
