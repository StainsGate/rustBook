#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut relation: Comparison;

    let first_length = _first_list.len();
    let second_length = _second_list.len();

    if first_length > second_length {
        relation = Comparison::Superlist;

        for i in 0..first_length {
            if second_length == 0 {
                break;
            }

            if _second_list[0] != _first_list[i] {
                continue;
            } else {
                let slice_len = i + second_length;

                if slice_len > first_length {
                    relation = Comparison::Unequal;
                } else {
                    if _first_list[i..slice_len] != _second_list[..] {
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
    } else {
        relation = Comparison::Sublist;

        if first_length == 0 && second_length == 0 {
            relation = Comparison::Equal;
        }

        if _first_list == _second_list {
            relation = Comparison::Equal;
        }

        for i in 0..second_length {
            if first_length == 0 {
                break;
            }

            if _first_list[0] != _second_list[i] {
                if second_length == 1 {
                    relation = Comparison::Unequal;
                }
                continue;
            } else {
                let slice_len = i + first_length;

                if slice_len > second_length {
                    relation = Comparison::Unequal;
                } else {
                    if _second_list[i..slice_len] != _first_list[..] {
                        continue;
                    } else {
                        break;
                    }
                }

                if slice_len == second_length && _second_list[i..slice_len] == _first_list[..] {
                    relation = Comparison::Equal;
                }
            }
        }
    }

    return relation;
}
