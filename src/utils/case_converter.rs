pub fn to_pascal_case(s: &str) -> String {
    s.split(|c| c == '-' || c == '_' || c == ' ')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}
