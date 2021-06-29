pub fn nth(n: u32) -> u32 {
    (2..).filter(|i| is_prime(*i)).nth(n as usize).unwrap_or(0)
}

fn is_prime(n: u32) -> bool {
    !(2..n).any(|i| n % i == 0)
}
