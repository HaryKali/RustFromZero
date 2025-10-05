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

// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
//     let s3 = s1 + &s2;
//     assert_eq!(s3,"hello,world!");
//     // 下面的语句如果去掉注释，就会报错
//     // println!("{}",s1);
// }


// 修复错误，不要新增代码行
// fn main() {
//     let s: &str = "hello, world";
// }


// 使用至少两种方法来修复错误
// fn main() {
//     let s: &str = "hello, world".into();
//     greetings(s)
// }
//
// fn greetings(s: &str) {
//     println!("{}",s)
// }

// 填空
// fn main() {
//     let mut s = String::from("");
//     s.push_str("hello, world");
//     s.push('!');
//
//     assert_eq!(s, "hello, world!");
// }


// 修复所有错误，并且不要新增代码行
// fn main() {
//     let mut s = String::from("hello");
//     s.push_str(",");
//     s.push_str(" world");
//     s += "!";
//
//     println!("{}", &s)
// }



// 填空
// fn main() {
//     let s = String::from("I like dogs");
//     // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
//     let s1 = s.replace("dogs", "cats");
//
//     assert_eq!(s1, "I like cats")
// }

// 填空

// 修复所有错误，不要删除任何一行代码
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1.clone() + &s2;
//     assert_eq!(s3,"hello,world!");
//     println!("{}",s1)
// }


// 使用至少两种方法来修复错误
// fn main() {
//     let s = "hello, world";
//     greetings(s.to_string())
// }
//
// fn greetings(s: String) {
//     println!("{}",s)
// }


// 使用至少两种方法来修复错误


// 使用两种方法来解决错误，不要新增代码行
// fn main() {
//     let s = "hello, world".to_string();
//     let s1 = s;
// }

// 使用两种方法来解决错误，不要新增代码行
// fn main() {
//     let s = "hello, world";
//     let s1: &str = s;
// }
/* 填空并修复所有错误 */
fn main() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // 修改上面的行让代码工作
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // 如果你希望在字符串中使用双引号，可以使用以下形式
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果希望在字符串中使用 # 号，可以如下使用：
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // 填空
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"")
}