#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // Helper function to check if `small` is a sublist of `big`
    fn is_sublist(small: &[i32], big: &[i32]) -> bool {
        if small.is_empty() {
            return true;
        }
        if small.len() > big.len() {
            return false;
        }

        for i in 0..=(big.len() - small.len()) {
            if &big[i..i + small.len()] == small {
                return true;
            }
        }
        false
    }

    // Now use that helper to decide what the relationship is
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
