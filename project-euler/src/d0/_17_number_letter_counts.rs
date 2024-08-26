use std::collections::HashMap;

fn _solution() -> usize {
    let map: HashMap<i32, &str> = HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
        (100, "hundred"),
        (1000, "thousand"),
    ]);

    let mut sum = 0;

    for i in 1..=1000 {
        let mut num = i;
        let mut num_str = String::new();

        if num >= 1000 {
            num_str.push_str(map.get(&(num / 1000)).unwrap());
            num_str.push_str(map.get(&1000).unwrap());

            num %= 1000;
        }

        if num >= 100 {
            num_str.push_str(map.get(&(num / 100)).unwrap());
            num_str.push_str(map.get(&100).unwrap());

            if num % 100 != 0 {
                num_str.push_str("and");
            }

            num %= 100;
        }

        if num >= 20 {
            num_str.push_str(map.get(&(num / 10 * 10)).unwrap());

            num %= 10;
        }

        if num > 0 {
            num_str.push_str(map.get(&num).unwrap());
        }

        sum += num_str.len();
    }

    sum
}

#[test]
fn test() {
    println!("{}", _solution());
}
