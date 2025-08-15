const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'x', 'y', 'w', 'z',
];

/// Converts a word to Pig Latin.
///
/// ## Example
/// ```rust
/// use pig_latin::to_pig_latin_word;
/// let word = "pantry";
/// assert_eq!(to_pig_latin_word(word), "antrypay");
/// ```
pub fn to_pig_latin_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next().unwrap() {
        first if VOWELS.contains(&first.to_ascii_lowercase()) => {
            // Word starts with a vowel:
            format!("{word}hay")
        }
        first if first.is_uppercase() => {
            let second = chars.next().unwrap();
            let rest: String = chars.collect();
            format!(
                "{}{}{}ay",
                second.to_uppercase(),
                rest,
                first.to_lowercase()
            )
        }
        first => {
            let rest: String = chars.collect();
            format!("{rest}{first}ay")
        }
    }
}

fn parse_word(word: &str) -> String {
    if word.len() > 1 && word.to_lowercase().chars().all(|c| ALPHABET.contains(&c)) {
        to_pig_latin_word(word)
    } else {
        word.to_string()
    }
}

/// Converts a sentence to Pig Latin.
///
/// ## Example
/// ```rust
/// use pig_latin::to_pig_latin_sentence;
/// let sentence = "The key is in the pantry";
/// assert_eq!(to_pig_latin_sentence(sentence), "Hetay eykay ishay inhay hetay antrypay");
/// ```
///
/// Words that have punctation adjoining them will not be
/// correctly translated i.e. "word." will be left untouched
/// rather than becoming "ordway."
pub fn to_pig_latin_sentence(sentence: &str) -> String {
    let words: Vec<String> = sentence.split_whitespace().map(parse_word).collect();
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pig_latin_word() {
        let first_letter_consonant = "pizza";
        let first_letter_vowel = "apple";

        assert_eq!(to_pig_latin_word(first_letter_consonant), "izzapay");
        assert_eq!(to_pig_latin_word(first_letter_vowel), "applehay");
    }

    #[test]
    fn pig_latin_capitalised_word() {
        let first_letter_consonant = "Pizza";
        let first_letter_vowel = "Apple";

        assert_eq!(to_pig_latin_word(first_letter_consonant), "Izzapay");
        assert_eq!(to_pig_latin_word(first_letter_vowel), "Applehay");
    }

    #[test]
    fn pig_latin_sentence() {
        let sentence = "The key is in the pantry";
        assert_eq!(
            to_pig_latin_sentence(sentence),
            "Hetay eykay ishay inhay hetay antrypay"
        );
    }

    #[test]
    fn pig_latin_sentence_with_punctuation() {
        let sentence = "First, you open the door.";
        assert_eq!(
            to_pig_latin_sentence(sentence),
            "First, ouyay openhay hetay door."
        );
    }
}
