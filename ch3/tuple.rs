// fn main(){
//     let tup:(i32,f64,u8)=(500,6.4,1);
//
//     let (x,y,z)=tup;
//     let a=tup.2;
//     println!("{} {}",x,a);
//
// }

// fn main() {
//     let mut s1 = String::from("hello ");
//     let s2 = "HI";
//     let (res, len) = calculate_length(& mut s1);
//     println!("The length of '{}' is {}, res is {}.", s1, len, res);
// }
//
// fn calculate_length(s: & mut String) -> (String, usize) {
//     let length = s.len();
//     s.push_str(" hello world");
//     (s.to_string(), length)
// }

// fn main() {
//     let _t0: (u8,i16) = (0, -1);
//     // 元组的成员还可以是一个元组
//     let _t1: (u8, (i16, u32)) = (0, (-1, 1));
//     // 填空让代码工作
//     let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
// }

// 修改合适的地方，让代码工作
// fn main() {
//     let t = ("i", "am", "sunface");
//     assert_eq!(t.2, "sunface");
// }

// 修复代码错误
// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5);
//     println!("too long tuple: {:?}", too_long_tuple);
// }

// fn main() {
//     let tup = (1, 6.4, "hello");
//
//     // 填空
//     let (x,z,y) = tup;
//
//     assert_eq!(x, 1);
//     assert_eq!(y, "hello");
//     assert_eq!(z, 6.4);
// }

// fn main() {
//     let (x, y, z);
//
//     // 填空
//     (y,z,x) = (1, 2, 3);
//
//     assert_eq!(x, 3);
//     assert_eq!(y, 1);
//     assert_eq!(z, 2);
// }



fn main() {
    // 填空，需要稍微计算下
    let (x, y) = sum_multiply((2,3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}