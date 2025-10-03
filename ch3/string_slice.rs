// fn main() {
//     let my_name = "Pascal";
//     greet(my_name);
// }
//
// fn greet(name: &str) {
//     println!("Hello, {}!", name);
// }

// fn main() {
//     let mut s = String::from("hello world");
//
//     let word = first_word(&s);
//
//     s.clear(); // error!
//
//     println!("the first word is: {}", word);
// }
// fn first_word(s: &String) -> &str {
//     &s[..1]
// }

// fn main(){
//     let s = String::from("hello world");
//
//     let hello = &s[0..5];
//     let world = &s[6..11];
//     println!("{}", &hello);
//
// }
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);
}
fn first_word(s: &String) -> &str {
    &s[..1]
}
