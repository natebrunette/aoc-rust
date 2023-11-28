pub fn group_on_empty(input: &Vec<String>) -> Vec<Vec<String>> {
    input
        .split(|line| line == "")
        .map(|slice| slice.to_vec())
        .collect()
}

pub fn vec_to_int(vec: &Vec<String>) -> Vec<i32> {
    vec.iter()
        .map(|string| string.parse::<i32>().unwrap())
        .collect()
}
