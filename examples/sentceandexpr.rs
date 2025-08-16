// fn add(x: i32, y: i32) -> i32{
//     let x = x+1;
//     let y = y+1;
//     x+y
//
//
// }
//
// fn main() {
//     // assert_eq!(return_unit_type(),());
//     println!("{}",return_unit_type());
//
// }
//
// fn return_unit_type()->  &'static str{
//     let x =1;
//     let y= if x%2 ==1 {
//         "odd"
//     } else{
//         "even"
//     };
//
//     let z = if x %2 ==1 {"odd"} else {"even"};
//
//     return z;
// }

// 使用两种方法让代码工作起来
//sol1
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//
//     };
//
//     assert_eq!(v, ());
// }

// 使用两种方法让代码工作起来
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
}


// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
//
//     println!("The value of y is: {}", y);
// }