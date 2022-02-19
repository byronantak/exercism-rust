use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let character_count_map = build_character_count_map(word);
    let mut result: HashSet<&'a str> = HashSet::new();
    for potential_anagram in possible_anagrams.iter() {
        if word.to_lowercase() == potential_anagram.to_lowercase() {
            continue;
        }

        if is_anagram(&character_count_map, *potential_anagram) {
            result.insert(*potential_anagram);
        }
    }
    result
}

fn build_character_count_map(word: &str) -> HashMap<char, i16> {
    let mut character_set: HashMap<char, i16> = HashMap::new();
    for c in word.to_lowercase().chars() {
        if character_set.contains_key(&c) {
            let current_count = character_set.get_mut(&c).unwrap();
            *current_count = *current_count + 1_i16;
        } else {
            character_set.insert(c, 1);
        }
    }
    character_set
}

fn is_anagram(character_count_map: &HashMap<char, i16>, expected_anagram: &str) -> bool {
    let mut character_count_down = character_count_map.clone();
    for c in expected_anagram.to_lowercase().chars() {
        match character_count_down.get_mut(&c) {
            Some(count) => {
                *count = *count - 1_i16;
            }
            None => {
                return false;
            }
        }
    }
    for (c, count) in character_count_down.iter() {
        println!("{} => {}", c, count);
        if *count != 0_i16 {
            return false;
        }
    }
    true
}
