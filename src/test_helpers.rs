pub fn vec_compare<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let match_count = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    match_count == a.len() && match_count == b.len()
}
