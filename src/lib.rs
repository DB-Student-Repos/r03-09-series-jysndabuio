pub fn series(digits: &str, len: usize) -> Vec<String> {
    let str_digits: Vec<char> = digits.chars().collect();
    let mut container = Vec::new();
    match len {
        0 => container.extend(vec!["".to_string(); digits.len()]), // Did not understand why it should return 6 empty string instead of 5.
        _ => container.extend(str_digits.windows(len).map(|x|x.iter().collect())),
    }
    container
}