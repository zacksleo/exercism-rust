pub fn factors(n: u64) -> Vec<u64> {
    let mut res = vec![];
    let num = n;
    if n <= 1 {
        return res;
    }
    let mut has_prime = false;
    for cur in 2..num {
        if num % cur == 0 {
            has_prime = true;
            res.push(cur);
            if num / cur > 1 {
                let mut res2 = factors(num / cur).clone();
                res.append(&mut res2);
                return res;
            }
        }
    }
    if !has_prime {
        res.push(n);
    }
    res
}

