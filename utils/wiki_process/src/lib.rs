pub mod wiki_parse {
    use regex::Regex;
    pub fn create_string_vector(text: &str) -> Vec<String> {
        let processed_text = process(text);
        let separators = [
            ' ', '\'', '.', '(', ')', ',', '!', '?', ';', ':', '/', '§', '%', '*', '€', ']', '[',
            '-', '\n',
        ];
        let separator_indexes: Vec<_> = [0]
            .into_iter()
            .chain(
                processed_text
                    .char_indices()
                    .filter_map(
                        |(index, char)| match separators.iter().find(|c| *c == &char) {
                            Some(_) => {
                                let num_bytes_char = char.len_utf8();
                                Some([index, index + num_bytes_char])
                            }
                            None => None,
                        },
                    )
                    .flatten(),
            )
            .collect();
        separator_indexes
            .windows(2)
            .map(|slice| {
                let start = *slice.get(0).expect("slice should have 2 elements");
                let end = *slice.get(1).expect("slice should have 2 elements");
                let chunk = &processed_text[start..end];
                let chunk_string = chunk.to_string();
                chunk_string
            })
            .map(|str| str.to_string())
            .collect()
    }

    pub fn process(s: &str) -> String {
        let no_dup_enter = s.replace("\n\n\n", "");
        let no_ier = no_dup_enter
            .replace("Ier", "Premier")
            .replace(" ier", " premier");
        let re = Regex::new(r#",{2,}"#).unwrap();
        re.replace_all(&no_ier, "").to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn commas_removed() {
        let test_string = "Hello !,,, Hi".to_string();
        assert_eq!(
            "Hello ! Hi".to_string(),
            crate::wiki_parse::process(&test_string)
        );
    }

    #[test]
    fn ier_removed() {
        let test_string = "Alexandre Ier".to_string();
        assert_eq!(
            "Alexandre Premier".to_string(),
            crate::wiki_parse::process(&test_string)
        );
    }
}
