pub fn is_valid(code: &str) -> bool {
    let code: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    if code.len() <= 1 {
        return false;
    }
    let mut has_no_digit = false;
    let mut final_result = false;

    code.chars().for_each(|c| {
        if !c.is_numeric() {
            has_no_digit = true
        }
    });

    if has_no_digit {
        return false;
    }

    let right_start_even_iter = code
        .chars()
        .enumerate()
        .filter(|&(i, _)| (code.len() - i) % 2 == 0)
        .map(|(_, c)| c.to_digit(10));

    let right_start_odd_iter = code
        .chars()
        .enumerate()
        .filter(|&(i, _)| (code.len() - i) % 2 != 0)
        .map(|(_, c)| c.to_digit(10));

    let mut even_sum = 0;
    let mut odd_sum = 0;

    for i in right_start_even_iter {
        match i {
            Some(value) => {
                if value * 2 > 9 {
                    even_sum += value * 2 - 9;
                } else {
                    even_sum += value * 2;
                }
            }
            None => {
                even_sum += 0;
            }
        }
    }

    for j in right_start_odd_iter {
        match j {
            Some(value) => {
                odd_sum += value;
            }
            None => {
                odd_sum += 0;
            }
        }
    }

    if (odd_sum + even_sum) % 10 == 0 {
        final_result = true;
    }

    return final_result;
}
