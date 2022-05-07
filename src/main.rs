const FERRIS_HEAD: &str = r#"    _~^~^~_
\) /  o o  \ (/"#;
const FERRIS_BODY: &str = "  |         |";
const FERRIS_BOTTOM: &str = r#"  '_   ¬   _'
  / '-----' \"#;

fn main() {
    let length = std::env::args()
        .nth(1)
        .expect("No length given")
        .parse::<usize>()
        .expect("Not integer given");
    println!("{}", FERRIS_HEAD);
    for _ in 0..length {
        println!("{}", FERRIS_BODY);
    }
    println!("{}", FERRIS_BOTTOM);
}
