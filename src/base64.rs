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
    for i in (0..=binary_sequence.len() - 6).step_by(6) {
        binary_group += &binary_sequence[i..i + 6];
        println!("{}", binary_group);
        let decimal_value: u8 = u8::from_str_radix(&binary_group, 2).unwrap();
        let base_64_value = BASE_64_CHARS.chars().nth(decimal_value as usize).unwrap();
        encoded_value.push(base_64_value);
        binary_group.clear();
    }
    if word.len() % 3 == 1{
        let incomplete_binary = binary_sequence[binary_sequence.len()-2..binary_sequence.len()].to_string() + "0000";
        let decimal_value: u8 = u8::from_str_radix(&incomplete_binary, 2).unwrap();
        let base_64_value = BASE_64_CHARS.chars().nth(decimal_value as usize).unwrap();
        encoded_value.push(base_64_value);
        encoded_value += "==";
    }
    else if word.len() % 3 == 2{
        let incomplete_binary = binary_sequence[binary_sequence.len()-4..binary_sequence.len()].to_string() + "00";
       println!("{}",incomplete_binary);
        let decimal_value: u8 = u8::from_str_radix(&incomplete_binary, 2).unwrap();
        let base_64_value = BASE_64_CHARS.chars().nth(decimal_value as usize).unwrap();
        encoded_value.push(base_64_value);
        encoded_value += "=";
    }
    return encoded_value;
}
