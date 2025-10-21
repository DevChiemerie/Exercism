use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    fn characters_sorted(s: &str) -> Vec<char> {
        let mut chars: Vec<char> = s.to_lowercase().chars().collect();
        chars.sort_unstable();
        chars
    }

    let target = characters_sorted(word);

    for &candidate in possible_anagrams {
        if candidate.to_lowercase() != word.to_lowercase()
            && characters_sorted(candidate) == target
        {
            result.insert(candidate);
        }
    }

    result
}
