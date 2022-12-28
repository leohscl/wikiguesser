pub fn same_root(candidate: &str, word: &str) -> bool {
    let l_candidate = candidate.len();
    let l_word = word.len();
    // check that words are equal for the first 60%
    let dist = std::cmp::max_by(l_candidate as f64, l_word as f64, |a, b| a.partial_cmp(b).unwrap());
    let min_len = std::cmp::min_by(l_candidate as f64, l_word as f64, |a, b| a.partial_cmp(b).unwrap());
    let len_equal_target = f64::ceil(dist * 0.6);
    let min_target_and_len = std::cmp::min_by(len_equal_target, min_len, |a, b| a.partial_cmp(b).unwrap());
    if min_len < 5 as f64 {
        return false;
    }
    let len_final = min_target_and_len as usize;
    let equal = word.chars().take(len_final)
        .zip(candidate.chars().take(len_final))
        .map(|(word_c, candidate_c)| {
            word_c == candidate_c
        })
        .all(|b| b);
    equal
}
