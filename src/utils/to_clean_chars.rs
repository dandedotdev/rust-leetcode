pub trait ToCleanChars {
    fn to_clean_chars(self) -> Vec<char>;
}

impl ToCleanChars for String {
    fn to_clean_chars(self) -> Vec<char> {
        self.to_ascii_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<Vec<_>>()
    }
}
