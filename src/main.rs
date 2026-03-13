mod base64;

fn main() {
     let word = "sim";
    println!("{}",base64::encode(word));
}
