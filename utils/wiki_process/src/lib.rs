pub mod wiki_parse {
    pub fn create_string_vector(text: &str) -> Vec<String> {
        let processed_text = text.replace("\n\n\n", "").to_string();
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
                let chunk = &text[start..end];
                let chunk_string = chunk.to_string();
                chunk_string
            })
            .map(|str| str.to_string())
            .collect()
    }
}
