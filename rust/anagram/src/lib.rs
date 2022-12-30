use std::collections::{HashSet, HashMap};

fn initialize_char_map(word: &str) -> HashMap<char, i32> {
    let mut char_map = HashMap::new();
    for c in word.chars() {
        *char_map.entry(c).or_insert(0) += 1
    }
    char_map
}

fn is_anagram(candidate: &str, char_map: &HashMap<char, i32>) -> bool {
    let mut map = char_map.clone();
    for c in candidate.chars() {
        match map.get_mut(&c) {
            None => return false,
            Some(count) =>
                if *count == 0 {
                    return false;
                } else {
                    *count -= 1;
                }
        }
    }

    for entry in map {
        if entry.1 != 0 {
            return false;
        } else {
            continue;
        }
    }

    true
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut results = HashSet::new();

    let lc_word = word.to_lowercase();
    let char_map = initialize_char_map(&lc_word);

    println!("word: {:?}", word);
    println!("char_map: {:?}", char_map);
    println!("possible_anagrams: {:?}", possible_anagrams);

    for pa_ref in possible_anagrams {
        let candidate = pa_ref.to_lowercase();

        if lc_word != candidate && is_anagram(&candidate, &char_map) {
            results.insert(*pa_ref);
        }
    }

    println!("result: {:?}", results);
    results
}
