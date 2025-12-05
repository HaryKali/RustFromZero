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

// struct Point<T>{
//     x:T,
//     y:T
// }
//
//
// impl<T: std::fmt::Display> Point<T>{
//     fn new(x:T,y:T)->Point<T>{
//         println!("New Point");
//         Point{x,y}
//     }
//
//     fn show_location(&self){
//         println!("Point show location {},{}",self.x,self.y);
//     }
// }
//
// fn main(){
//     let point1 = Point::new(1,2);
//     point1.show_location();
//
//
// }

// struct Point<T>{
//     x:T,
//     y:T
// }
//
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// fn main(){
//     let p= Point{x:1,y:2};
//     println!("{:?}",p.x());
//
//
// }
// struct Point<T> {
//     x: T,
//     y: T
// }
//
// struct MixPoint<U, W> {
//     x: U,
//     y: W
// }
//
// impl<U, W> MixPoint<U, W> {
//     fn mix<'a, T>(&'a self, other: &'a Point<T>) -> MixPoint<&'a T, &'a W> {
//         MixPoint {
//             x: &other.x,
//             y: &self.y
//         }
//     }
// }
//
// fn main() {
//     let p1 = Point { x: 1, y: 2 };
//     let p2 = MixPoint { x: "Hello", y: 3 };
//     let p3 = p2.mix(&p1);
//
//     println!("p2.x: {}", p2.x);     // 输出: Hello
//     println!("p3.x: {}, p3.y: {}", p3.x, p3.y); // 输出: 1, 3
// }
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
//
// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c'};
//
//     let p3 = p1.mixup(p2);
//
//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// fn display_array(arr: [String; 3]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr: [String; 3] = ["hi".to_string(), "hello".to_string(), "hey".to_string()];
//     display_array(arr);
//     // for num in arr.iter() {
//     //     println!("{}", num);
//     // }
//     let arr: [String; 2] = ["123".to_string(), "23".to_string()];
//
//
//
//     display_array(arr);
// }

// fn display_array(arr: &[i32]){
//     println!("{:?}", arr);
// }
//
// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     display_array(&arr);
//
//     let arr: [i32; 3] = [0; 3];
//
//     display_array(&arr);
//
// }

// fn display_array<T: std::fmt::Debug>(arr: &[T]){
//     println!("{:?}", arr);
// }
//
// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     display_array(&arr);
//
//     let arr: [&str; 3] = ["hi"; 3];
//
//     display_array(&arr);
//
// }

// fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T;N]) {
//     println!("{:?}", arr);
// }
//
// fn main() {
//     // let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     // display_array(arr);
//
//     let arr: [&str; 3] = ["hi"; 3];
//
//     display_array(arr);
// }
//111