mod base64;

fn main() {
    let word = "test value";
    let encoded_value = base64::encode(word);
    println!("encoded Value = {}",encoded_value);
    print!("Decoded value = {}", base64::decode(&encoded_value));
}
