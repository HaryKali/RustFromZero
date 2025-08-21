// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x = String::from("hello, world");
//     let y = &x;
//     println!("{},{}", x, y);
// }
// 不要修改 main 中的代码
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);
//
//     println!("{}", s2);
// }
//
// // 只能修改下面的代码!
// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }

// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }
//
// // 只能修改下面的代码!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     // 将 String 转换成 Vec 类型
//     // let _s = s.into_bytes();
//     s
// }


// fn main() {
//     let s = String::from("hello, world");
//     print_str(&s); // Pass a reference instead of moving ownership
//
//     println!("{}", s); // 's' remains valid here
// }
//
// fn print_str(s1: &str) { // Accept a string slice (&str) instead of String
//     println!("{}", s1); // Directly use the reference
// }
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = &x;
    println!("{:?}, {:?}", x, y);
}