#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.is_empty() && _second_list.is_empty() {
        return Comparison::Equal;
    } else if _first_list.is_empty() {
        return Comparison::Sublist;
    } else if _second_list.is_empty() {
        return Comparison::Superlist;
    }
    if _first_list.len() == _second_list.len() {
        match _first_list == _second_list {
            true => return Comparison::Equal,
            false => return Comparison::Unequal,
        }
    } else if _first_list.len() > _second_list.len() {
        // [0,1,2,3,4,5,6]
        // [1,2,3]
        for i in 0..(_first_list.len()-_second_list.len()+1) {
           if &_first_list[i..i+_second_list.len()] == _second_list {
               return Comparison::Superlist;
           }
        }
        return Comparison::Unequal;
    }
    for i in 0..(_second_list.len()-_first_list.len()+1) {
        if &_second_list[i..i+_first_list.len()] == _first_list {
            return Comparison::Sublist;
        }
     }
     return Comparison::Unequal;
}
