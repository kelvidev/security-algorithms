mod base64;

fn main() {
     let word = "thats a test";
    println!("{}",base64::encode(word));
}
