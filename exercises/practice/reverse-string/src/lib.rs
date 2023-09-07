pub fn reverse(input: &str) -> String {
    let mut vec: Vec<char> = input.chars().collect();
    vec.reverse();
    return vec.into_iter().collect()
}
