pub fn group_on_empty(input: &Vec<String>) -> Vec<Vec<String>> {
    input
        .split(|line| line == "")
        .map(|slice| slice.to_vec())
        .collect()
}

pub fn vec_to_int(vec: &Vec<String>) -> Vec<usize> {
    vec.iter()
        .map(|string| string.parse::<usize>().unwrap())
        .collect()
}
