use std::collections::HashSet;

fn to_sorted_string(a: &str) -> String {
    let mut a: Vec<char> = a.chars().collect();
    a.sort();
    a.iter().collect()
}

fn is_anagram(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a = a.to_lowercase();
    let b = b.to_lowercase();

    if a == b {
        return false;
    }

    let a = to_sorted_string(&a);
    let b = to_sorted_string(&b);

    a == b
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&&candidate| is_anagram(word, candidate))
        .copied()
        .collect()
}
