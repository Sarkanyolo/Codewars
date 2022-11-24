// https://www.codewars.com/kata/541c8630095125aba6000c00/train/rust

pub fn digital_root(n: i64) -> i64 {
    let mut str = n.to_string();
    let mut sum : i64 = 0;
    while str.len() > 1  {
        let charsum = sum_of_chars(&str);
        sum = i64::from(charsum);
        str = charsum.to_string();
    }

    sum
}

fn sum_of_chars(s : &str) -> u32 {
    s.chars().fold(0, |acc, c| c.to_digit(10).unwrap_or(0) + acc)
}

pub fn test_digital_root() -> bool {
    if digital_root(16) != 7 { println!("1"); return false; }
    if digital_root(942) != 6 { println!("2"); return false; }
    if digital_root(132189) != 6 { println!("3"); return false; }
    if digital_root(493193) != 2 { println!("4"); return false; }
    true
}