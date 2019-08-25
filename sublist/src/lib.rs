use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq + Debug>(list: &[T], sublist: &[T]) -> bool {
    let sublist_len = sublist.len();
    if sublist_len == 0 {
        return true;
    }

    list.windows(sublist_len).any(|l| l == sublist)
}

pub fn sublist<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal if first_list == second_list => Comparison::Equal,
        Ordering::Greater if is_sublist(first_list, second_list) => Comparison::Superlist,
        Ordering::Less if is_sublist(second_list, first_list) => Comparison::Sublist,
        _ => Comparison::Unequal
    }
}
