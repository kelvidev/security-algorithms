const BASE_64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(data: &str) -> String {
    let mut binary_sequence = String::new();
    for simbol in data.chars() {
        let binary_value = format!("{:08b}", simbol as u32);
        binary_sequence += &binary_value;
    }
    println!("{}", binary_sequence);
    // let sequence_list: Vec<char> = binary_sequence.chars().collect();
    let mut encoded_value = String::new();
    let mut binary_group = String::new();
    for i in (0..=binary_sequence.len() - 6).step_by(6) {
        binary_group = String::from(&binary_sequence[i..i + 6]);
        let decimal_value: u8 = u8::from_str_radix(&binary_group, 2).unwrap();
        let base_64_value = BASE_64_CHARS.chars().nth(decimal_value as usize).unwrap();
        encoded_value.push(base_64_value);
        binary_group.clear();
    }
    if data.len() % 3 == 1{
        let incomplete_binary = binary_sequence[binary_sequence.len()-2..binary_sequence.len()].to_string() + "0000";
        let decimal_value: u8 = u8::from_str_radix(&incomplete_binary, 2).unwrap();
        let base_64_value = BASE_64_CHARS.chars().nth(decimal_value as usize).unwrap();
        encoded_value.push(base_64_value);
        encoded_value += "==";
    }
    else if data.len() % 3 == 2{
        let incomplete_binary = binary_sequence[binary_sequence.len()-4..binary_sequence.len()].to_string() + "00";
        let decimal_value: u8 = u8::from_str_radix(&incomplete_binary, 2).unwrap();
        let base_64_value = BASE_64_CHARS.chars().nth(decimal_value as usize).unwrap();
        encoded_value.push(base_64_value);
        encoded_value += "=";
    }
    return encoded_value;
}

pub fn decode(data: &str) -> String {

    let equals_amount = data.matches('=').count();
    let formated_data = data.replace("=", "");

    let mut binary_sequence = String::new();

    for simbol in formated_data.chars() {
        let decimal_value: u8 = BASE_64_CHARS.find(simbol).unwrap() as u8; 
        let binary_value = format!("{:06b}", decimal_value );
        binary_sequence += &binary_value;
    }
    if equals_amount == 1{
        binary_sequence.truncate(binary_sequence.len() - 2);
    }
    else if equals_amount == 2{
        binary_sequence.truncate(binary_sequence.len() - 4);
    }
    let mut decoded_value:String = String::from("");
    for i in (0..=binary_sequence.len() - 8).step_by(8) {
        let binary = u8::from_str_radix(&binary_sequence[i..i+8], 2).unwrap();
        decoded_value += &(binary as char).to_string();
    }


    return decoded_value;
}
