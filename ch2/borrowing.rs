//
// fn main() {
//     let x = 5;
//     let p = &x;
//
//     println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
// }


// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");
//
//     borrow_object(&mut s)
// }
//
// fn borrow_object(s: &String) {}


// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");
//
//     println!("{}",push_str(& mut s))
// }
//
// fn push_str(s: &mut String) ->  String{
//     s.push_str("world");
//     s.to_string()
// }


// fn main() {
//     let c = '中';
//
//     let r1 = &c;
//     // 填写空白处，但是不要修改其它行的代码
//     let ref r2 = c;
//
//     assert_eq!(*r1, *r2);
//
//     // 判断两个内存地址的字符串是否相等
//     assert_eq!(get_addr(r1),get_addr(r2));
// }
//
// // 获取传入引用的内存地址的字符串形式
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }


// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
// fn main() {
//     let s = String::from("hello");
//
//     let r1 = &s;
//     let r2 = &s;
//
//     println!("{}, {}", r1, r2);
// }



// 注释掉一行代码让它工作
// fn main() {
//     let mut s = String::from("hello, ");
//
//     let r1 = &mut s;
//     r1.push_str("world");
//     // let r2 = &mut s;
//     r1.push_str("!");
//
//     println!("{}",r1);
// }

// fn main() {
//     let mut s = String::from("hello, ");
//
//     let r1 = &mut s;
//     let r2 = &mut s;
//
//     // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
//     // 你不能同时使用 r1 和 r2
//     println!("{}",r1,r2)
//
// }