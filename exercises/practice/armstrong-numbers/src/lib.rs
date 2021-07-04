pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    let len = str.len() as u32;
    let sum: u32 = str.chars().map(|e| e.to_digit(10).unwrap().pow(len)).sum();
    sum == num
}
