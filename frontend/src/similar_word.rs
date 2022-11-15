
pub fn same_root(candidate: &str, word: &str) -> bool {
    let l_candidate = candidate.len();
    let l_word = word.len();
    // check that words are equal for the first 60%
    let min_dist = std::cmp::min_by(l_candidate as f64, l_word as f64, |a, b| a.partial_cmp(b).unwrap());
    let len_equal_target = f64::ceil(min_dist * 0.75) as usize;
    let equal = word.chars().take(len_equal_target)
        .zip(candidate.chars().take(len_equal_target))
        .map(|(word_c, candidate_c)| {
            word_c == candidate_c
        })
        .all(|b| b);
    equal
}
