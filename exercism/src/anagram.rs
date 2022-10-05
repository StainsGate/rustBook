use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let mut origin_chars: Vec<char> = Vec::new();
    let lowcase_origin = word.to_lowercase();

    for i in possible_anagrams.iter() {
        let lowcase_compare = i.to_lowercase();

        for char in lowcase_origin.chars() {
            origin_chars.push(char);
        }

        let length = word.len();

        if i.len() != length {
            origin_chars.clear();
            continue;
        }

        for j in lowcase_compare.chars() {
            if origin_chars.contains(&j) {
                // compare_chars.push(j);
                let pos = origin_chars.iter().position(|x| *x == j).unwrap();
                origin_chars.remove(pos);
            } else {
                break;
            }
        }

        if origin_chars.is_empty() && lowcase_origin != lowcase_compare {
            result.insert(i);
        }

        origin_chars.clear();
    }

    result.remove(word);
    return result;
}
