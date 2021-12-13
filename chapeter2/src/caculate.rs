pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut temp_result: Vec<i32> = Vec::new();
    let mut stack: Vec<&CalculatorInput> = Vec::new();
    // 4 8 + 7 5 - /
    let iter = inputs.iter().rev();

    for item in iter {
        stack.push(item)
    }

    while stack.len() > 0 {
        let tem1 = stack.pop().unwrap();

        match tem1 {
            CalculatorInput::Add => {
                // too few operands
                if temp_result.len() < 2 {
                    return None;
                }

                let right = temp_result.pop();
                let left = temp_result.pop();
                temp_result.push(left.unwrap() + right.unwrap())
            }
            CalculatorInput::Subtract => {
                // too few operands
                if temp_result.len() < 2 {
                    return None;
                }

                let right = temp_result.pop();
                let left = temp_result.pop();
                temp_result.push(left.unwrap() - right.unwrap())
            }
            CalculatorInput::Multiply => {
                // too few operands
                if temp_result.len() < 2 {
                    return None;
                }

                let right = temp_result.pop();
                let left = temp_result.pop();
                temp_result.push(left.unwrap() * right.unwrap())
            }
            CalculatorInput::Divide => {
                // too few operands
                if temp_result.len() < 2 {
                    return None;
                }
                let right = temp_result.pop();
                let left = temp_result.pop();
                temp_result.push(left.unwrap() / right.unwrap())
            }
            CalculatorInput::Value(x) => temp_result.push(*x),
        }
    }

    // zero operands
    if temp_result.is_empty() {
        return None;
    }
    // too many operands
    else if temp_result.len() > 1 {
        return None;
    }
    // normal
    else {
        return temp_result.pop();
    }
}

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut wordMap: HashMap<&str, i32> = HashMap::new();
    let mut result: bool;

    for iter in magazine.iter() {
        *wordMap.entry(*iter).or_insert(0) += 1
    }

    for iter_note in note.iter() {
        *wordMap.entry(iter_note).or_insert(0) -= 1;
    }

    let negative = wordMap.into_iter().find(|(_, v)| v < &0);

    match negative {
        None => {
            result = true;
        }

        _ => result = false,
    }

    return result;
}
