use std::fs;
use rand::Rng;

pub fn get_random_entry_from_file(file_path: &str) -> String {
    let word_list = convert_file_to_vec(file_path);
    get_random_entry(word_list)
}

fn convert_file_to_vec(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");
    return contents
        .lines()
        .map(|line| String::from(line))
        .collect();
}

fn get_random_entry (list: Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..list.len());

    String::from(list.get(random_index).unwrap())
}