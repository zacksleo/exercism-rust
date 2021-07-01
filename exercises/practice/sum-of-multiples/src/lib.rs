pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| is_factor(*x, factors))
        .into_iter()
        .fold(0, |sum, x| sum + x)
}

fn is_factor(n: u32, factors: &[u32]) -> bool {
    factors.iter().any(|x| *x != 0 && n % x == 0)
}

#[test]
fn test_zero() {
    assert_eq!(false, is_factor(1, &[0]));
}

#[test]
fn test_empty(){
    assert_eq!(false, is_factor(10000, &[]));
}
