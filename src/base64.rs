const BASE_64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(word: &str) -> String {
    let mut binary_sequence = String::new();
    for simbol in word.chars() {
        let binary_value = format!("{:08b}", simbol as u32);
        binary_sequence += &binary_value;
    }
    println!("{}", binary_sequence);
    // let sequence_list: Vec<char> = binary_sequence.chars().collect();
    let mut encoded_value = String::new();
    let mut binary_group = String::new();
    for i in (0..binary_sequence.len()).step_by(6) {
        binary_group += &binary_sequence[i..i + 6];
        println!("{}", binary_group);
        let decimal_value: u8 = u8::from_str_radix(&binary_group, 2).unwrap();
        let base_64_value = BASE_64_CHARS.chars().nth(decimal_value as usize).unwrap();
        encoded_value.push(base_64_value);
        binary_group.clear();
    }
    return encoded_value;
}
