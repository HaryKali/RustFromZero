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
// fn main() {
//     let raw_str = "Escapes don't work here: \x3F \u{211D}";
//     // 修改上面的行让代码工作
//     assert_eq!(raw_str, "Escapes don't work here: ? ℝ");
//
//     // 如果你希望在字符串中使用双引号，可以使用以下形式
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);
//
//     // 如果希望在字符串中使用 # 号，可以如下使用：
//     let  delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);
//
//     // 填空
//     let long_delimiter = r###"Hello, "##""###;
//     assert_eq!(long_delimiter, "Hello, \"##\"")
// }

//slice


// 修复代码中的错误，不要新增代码行!
// fn main() {
//     let arr = [1, 2, 3];
//     let s1: &[i32] = &arr[0..2];
//
//     let s2: &str = "hello, world" as &str;
// }


// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];
//     let slice_pre = String::from("Hello world");
//     let slice = &slice_pre[..3];
//
//     // 修改数字 `8` 让代码工作
//     // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
//     assert!(std::mem::size_of_val(&slice) == 18);
// }


// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];
//
//     let slice = &arr[..2];
//
//     // 修改数字 `8` 让代码工作
//     // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
//     assert!(std::mem::size_of_val(&slice) == 8);
// }


// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     // 填空让代码工作起来
//     let slice:&[i32]  =&arr[1..4];
//     assert_eq!(slice, &[2, 3, 4]);
// }


// fn main() {
//     let s = String::from("hello");
//
//     let slice1 = &s[0..2];
//     // 填空，不要再使用 0..2
//     let slice2 = &s[..2];
//
//     assert_eq!(slice1, slice2);
// }

// fn main() {
//     let s = "你好，世界";
//     // 修改以下代码行，让代码工作起来
//     let slice = &s[0..3];
//
//     assert!(slice == "你");
// }


// 修复所有错误
fn main() {
    let mut s = String::from("hello world");

    // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
    // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
    let ch = first_character(&s);

    s.to_string().clear(); // error!

    println!("the first character is: {}", ch);
}
fn first_character(s: &str) -> &str {
    &s[..1]
}