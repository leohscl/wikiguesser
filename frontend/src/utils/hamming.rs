pub fn same_root(candidate: &str, word: &str) -> bool {
    let l_candidate = candidate.len();
    let l_word = word.len();
    let distance = hamming_with_normal_size(candidate, word);
    let distance_f64 = distance as f64;
    let min_dist = std::cmp::max_by(l_candidate as f64, l_word as f64, |a, b| a.partial_cmp(b).unwrap());
    match min_dist {
        x if x == 0.0 => false,
        _ => {
            let normalized_dist = distance_f64 / min_dist;
            println!("normalized_dist: {}", normalized_dist);
            normalized_dist < 0.4
        }
    }
}
pub fn hamming_with_normal_size(candidate: &str, word: &str) -> usize {
    let l_candidate = candidate.len();
    let l_word = word.len();
    // let mut word = word.clone();
    // TODO(leo: handle errors)
    let (candidate_cmp, word_cmp) = if l_candidate < l_word {
        let padding = l_word - l_candidate;
        let candidate_cmp = candidate.chars().chain(std::iter::repeat(' ').take(padding)).collect::<String>();
        (candidate_cmp, word.to_string())
    } else {
        let padding = l_candidate - l_word;
        let word_cmp: String = word.chars().chain(std::iter::repeat(' ').take(padding)).collect();
        (candidate.to_string(), word_cmp)
    };
    // println!("Compared strings: {}, {}", candidate_cmp, word_cmp);
    // println!("lenghts: {},{}", candidate_cmp.len(), word_cmp.len());
    let res_distance = hamming(&candidate_cmp, &word_cmp);
    // println!("res_distance: {:?}", res_distance);
    res_distance.unwrap()
}
fn hamming(candidate: &str, word: &str) -> Result<usize, String> {
    match candidate.chars().count() == candidate.chars().count() {
        true => {
            let dist = candidate.chars()
                .zip(word.chars())
                .map(|(c_candidate, c_word)|{
                    println!("Compared chars: {}, {}", c_candidate, c_word);
                    match c_candidate == c_word {
                        false => 1,
                        true => 0,
                    }
                })
                .sum();
            Ok(dist)
        },
        false => Err("Error !".to_string())
    }
}
