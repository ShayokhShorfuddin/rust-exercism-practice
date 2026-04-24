/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let mut distance: usize = 0;

    if s1.len() != s2.len() {
        return None;
    }

    let characters_of_s2: Vec<char> = s2.chars().collect();
    for (index, letter) in s1.chars().enumerate() {
        if letter != characters_of_s2[index] {
            distance += 1;
        }
    }

    Some(distance)
}
