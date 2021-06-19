use std::collections::HashSet;

fn sort(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let sorted = sort(&word);
    possible_anagrams
        .iter()
        .filter(|e| {
            let x = e.to_lowercase();
            x != word && sorted == sort(&x)
        })
        .cloned()
        .collect()
}
