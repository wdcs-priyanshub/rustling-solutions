// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            first.to_uppercase().collect::<String>() + chars.as_str()
        }
    }
    //let first_cap = first_lower.to_uppercase();
    //input.replace(first_lower.as_str(),first_cap.as_str())
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
    words.iter().map(|word| {
        let mut chars = word.chars();
        match chars.next() {
        None => String::new(),
        Some(first) => {first.to_uppercase().collect::<String>() + chars.as_str() }
    }}).collect()
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
    let v = words.iter().map(|word| {
        let mut chars = word.chars();
        match chars.next() {
        None => String::new(),
        Some(first) => {first.to_uppercase().collect::<String>() + chars.as_str() }
    }}).collect::<String>();

    v.to_string()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
