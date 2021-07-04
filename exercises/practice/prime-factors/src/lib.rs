pub fn factors(n: u64) -> Vec<u64> {
    let mut res = vec![];

    match (2..n + 1).find(|x| n % x == 0) {
        Some(x) => {
            res.push(x);
            res.append(&mut factors(n / x));
        }
        None if n != 1 => {
            res.push(n);
        }
        _ => {}
    }
    res
}
