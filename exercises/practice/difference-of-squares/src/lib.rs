pub fn square_of_sum(n: u32) -> u32 {
    (1 + n) * n  * (1 + n) * n / 4
}

pub fn sum_of_squares(n: u32) -> u32 {
    (0..n + 1).fold(0, |sum, i| sum + i*i)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
