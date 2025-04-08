fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|w| capitalize_first(w)).collect()
}

fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|w| capitalize_first(w)).collect()
}
