use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();

    let mut word_chars: Vec<char> = word_lower.chars().collect();
    word_chars.sort();

    let mut anagrams: HashSet<&str> = Default::default();

    for anagram in possible_anagrams {
        let anagram_lower = anagram.to_lowercase();
        let mut anagram_chars: Vec<char> = anagram_lower.chars().collect();
        anagram_chars.sort();

        if anagram_chars == word_chars && anagram_lower != word_lower {
            anagrams.insert(anagram.clone());
        }
    }

    anagrams
}
