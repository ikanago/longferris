const FERRIS_HEAD: &'static str = r#"    _~^~^~_
\) /  o o  \ (/"#;
const FERRIS_BODY: &'static str = "  |         |";
const FERRIS_BOTTOM: &'static str = r#"  '_   ¬   _'
  / '-----' \"#;

fn main() {
    let n = 3;
    println!("{}", FERRIS_HEAD);
    for _ in 0..n {
        println!("{}", FERRIS_BODY);
    }
    println!("{}", FERRIS_BOTTOM);
}
