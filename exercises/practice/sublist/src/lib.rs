#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() < b.len() {
        return false;
    }
    if a.starts_with(b) {
        return true;
    }
    contains(&a[1..], b)
}

// fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
//     if a.len() < b.len() {
//         return false;
//     }
//     for i in 0..a.len() {
//         if a[i..].starts_with(b) {
//             return true;
//         }
//     }
//     false
// }

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        return Comparison::Equal;
    } else if contains(a, b) {
        return Comparison::Superlist;
    } else if contains(b, a) {
        return Comparison::Sublist;
    }
    return Comparison::Unequal;
}
