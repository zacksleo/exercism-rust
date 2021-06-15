pub fn reverse(input: &str) -> String {
    let res: String = input.chars().into_iter().rev().collect();
    res
}
