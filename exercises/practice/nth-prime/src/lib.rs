pub fn nth(n: u32) -> u32 {
    let mut num = 2;
    let mut i = 0;
    loop {
        if is_prime(num) {
            if n == i {
                return num;
            }
            i += 1;
        } else {
        }
        num += 1;
    }
}

fn is_prime(n: u32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
