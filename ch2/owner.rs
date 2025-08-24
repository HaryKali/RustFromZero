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
// fn main() {
//     let x = (1, 2, (), "hello".to_string());
//     let y = &x;
//     println!("{:?}, {:?}", x, y);
// }


// fn main() {
//     let s = String::from("hello, ");
//
//     // 只修改下面这行代码 !
//     let mut s1 = s;
//
//     s1.push_str("world");
//     println!("{}",s1);
// }


// fn main() {
//     let x = Box::new(5);
//
//     let mut y=Box::new(4);      // 完成该行代码，不要修改其它行！
//
//     *y = 4;
//
//     assert_eq!(*x, 5);
// }

// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//
//     let _s = t.0;
//
//     // 仅修改下面这行代码，且不要使用 `_s`
//     println!("{:?}", t.1);
// }

// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//
//     // 填空，不要修改其它代码
//     let (__, __) = __;
//
//     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
// }

fn main() {
    let t = (String::from("hello"), String::from("world"));

    // 填空，不要修改其它代码
    let (s1, s2) = (&t.0,&t.1);

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}