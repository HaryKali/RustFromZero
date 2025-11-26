// fn add_i8(a:i8, b:i8) -> i8 {
//     a + b
// }
// fn add_i32(a:i32, b:i32) -> i32 {
//     a + b
// }
// fn add_f64(a:f64, b:f64) -> f64 {
//     a + b
// }
//
// fn main() {
//     println!("add i8: {}", add_i8(2i8, 3i8));
//     println!("add i32: {}", add_i32(20, 30));
//     println!("add f64: {}", add_f64(1.23, 1.23));
// }

// fn add<T:std::cmp::PartialOrd>(a: T, b: T) -> T{
//     a+b
// }
//
// fn main() {
//     println!("add i8 {}", add(1i8, 2i8));
//     println!("add i16 {}", add(1, 2));
//     println!("add f64 {}", add(1.01, 2.02));
//
// }

// fn largest<T:std::cmp::PartialOrd>(list: &[T]) -> &T{
//     let mut largest = &list[0];
//     for &item in list.iter(){
//         if item > largest{
//             largest = item;
//         }
//     }
//
//     largest
//
// }
//
//
// fn main(){
//     let nums=vec![34, 50, 25, 100, 65];
//     let result=largest(&nums);
//     dbg!(result);
//
//
//
// }

// use std::fmt::Display;
//
// fn create_and_print<T>() where T: From<i32> + Display {
//     let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
//     println!("a is: {}", a);
// }
//
// fn main() {
//     create_and_print::<i64>();
// }

// struct Point2D<T> {
//     x:T,
//     y:T
// }
//
// fn main() {
//     let integer=Point2D{x:0,y:0};
//     let float=Point2D{x:0.1,y:0.2};
//
//
// }

// struct Point<T,U>{
//     x:T,
//     y:U
// }
// fn main(){
//     let p=Point{x:1,y:2.2};
// }

// enum Option<T> {
//     Some(T),
//     None,
// }
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }


struct Point<T>{
    x:T,
    y:T
}


impl<T: std::fmt::Display> Point<T>{
    fn new(x:T,y:T)->Point<T>{
        println!("New Point");
        Point{x,y}
    }

    fn show_location(&self){
        println!("Point show location {},{}",self.x,self.y);
    }
}

fn main(){
    let point1 = Point::new(1,2);
    point1.show_location();


}