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
// fn main() {
//     let mut s = String::from("hello world");
//
//     let word = first_word(&s);
//
//     // s.clear(); // error!
//
//     println!("the first word is: {}", word);
// }
// fn first_word(s: &String) -> &str {
//     &s[..1]
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let slice = &a[1..3];
//
//     println!("{}", slice[0]);
//
//
// }

// fn main() {
//     let mut s = String::from("HI");
//     let s1= String::from("Hello");
//     s.push_str("rust");
//
//     println!("Push Str {}",&s);
//
//
//     println!("{}",&s1);
//
//
//
//
// }

// fn main() {
//     let mut s = String::from("Hello rust!");
//     s.insert(5, ',');
//     println!("插入字符 insert() -> {}", s);
//     s.insert_str(6, " I like");
//     println!("插入字符串 insert_str() -> {}", s);
// }

// fn main() {
//     let string_replace = String::from("I like rust. Learning rust is my favorite!");
//     // let new_string_replace = string_replace.replace("rust", "RUST");
//     let new_string_replace= string_replace.replacen("rust","py",2);
//     dbg!(new_string_replace) ;
// }

// fn main() {
//     let mut string_replace_rangte=String::from("aaaa");
//     string_replace_rangte.replace_range(1..3,"b");
//     dbg!(&string_replace_rangte);
//
//
//
// }

// fn main() {
//     let mut string_pop=String::from("rust pop 中文！");
//     let p1 = string_pop.pop();
//     let p2 = string_pop.pop();
//     dbg!(p1, p2);
//     dbg!(string_pop);
//
// }

// fn main() {
//     let mut string_remove = String::from("测试remove方法");
//     println!(
//         "string_remove 占 {} 个字节",
//         std::mem::size_of_val(string_remove.as_str())
//     );
//     // 删除第一个汉字
//     // string_remove.remove(0);
//     // 下面代码会发生错误
//     // 直接删除第二个汉字
//     string_remove.remove(3);
//     dbg!(string_remove);
// }

// fn main() {
//     let mut string_truncate = String::from("测试truncate");
//     string_truncate.truncate(7);
//     dbg!(string_truncate);
// }
// fn main() {
//     let string_append = String::from("hello ");
//     let string_rust = String::from("rust");
//     // &string_rust会自动解引用为&str
//     let result = string_append + &string_rust;
//     let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
//     result += "!!!";
//
//     println!("连接字符串 + -> {}", result);
// }fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
//     let s3 = s1 + &s2;
//     assert_eq!(s3,"hello,world!");
//     // 下面的语句如果去掉注释，就会报错
//     // println!("{}",s1);
// }

fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
}