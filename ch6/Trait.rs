// pub trait Summary {
//     fn summarize(&self) -> String;
// }
// pub struct Post {
//     pub title: String,
//     pub author: String,
//     pub content: String,
// }
//
// pub struct Weibo{
//     pub content: String,
//     pub username: String,
// }
//
// impl Post{
//     fn init(){
//         println!("init post");
//     }
// }
//
//
// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.title, self.author)
//     }
// }
//
// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} by {}", self.content, self.username)
//     }
// }
//
// fn main() {
//     let post= Post{title: String::from("Test"), author: String::from("Michael"), content: String::from("Hello World!") };
//     let weibo = Weibo{content: String::from("Hello World!"), username: String::from("Michael") };
//     println!("weibo content: {}", weibo.summarize());
//     println!("Post username: {}", post.summarize());
// }

// pub trait Summary {
//     fn summarize_author(&self) -> String;
//
//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_author())
//     }
// }
//
//
// // impl Summary for Post{}
// impl Summary for Weibo{
//     fn summarize_author(&self) -> String{
//         format!("@{}", self.username)
//     }
// }
// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("{} by {}", self.title, self.author)
//     }
//
//     fn summarize_author(&self) -> String{
//         format!("@{}", self.author)
//     }
// }
//
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
//
// pub fn notify2<T: Summary>(item: &T){
//
//     println!("Breaking news! {}", item.summarize());
// }
//
// pub fn notify3<T: Summary>(item1: &T, item2: &T){
//     println!("Item1：Breaking news! {}", item1.summarize());
//     println!("Item2: Breaking news! {}", item2.summarize());
// }
// pub fn notify4(item1: &impl Summary, item2: &impl Summary){
//     println!("Item1：Breaking news! {}", item1.summarize());
//     println!("Item2: Breaking news! {}", item2.summarize());
// }
//
//
// fn main() {
//     let post= Post{title: String::from("Test"), author: String::from("Michael"), content: String::from("Hello World!") };
//     let weibo = Weibo{content: String::from("Hello World!"), username: String::from("Michael") };
//     // println!("{}", post.summarize());
//     // println!("{}", weibo.summarize());
//     // notify(&weibo);
//     // notify2(&weibo);
//     // notify3(&weibo,&post);
//     notify4(&weibo,&post);
//
// }
// use std::fmt::Display;
//
// struct Pair<T,Z> {
//     x: T,
//     y: Z,
// }
//
// impl<T,Z> Pair<T,Z> {
//     fn new(x: T, y: Z) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }
//
// impl<T: Display + PartialOrd,Z> Pair<T,Z> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// fn main(){
//     let test_pair=Pair{x: 'c', y: 3.11};
//     test_pair.cmp_display();
//
// }
// fn largest<T:PartialOrd+ Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// use std::ops::Add;
//
// #[derive(Debug)]
//
// struct Point<T: Add<T,Output = T>>{
//     x:T,
//     y:T,
//
// }
//
// impl <T: Add<T,Output = T>> Add for Point<T> {
//     type Output = Point<T>;
//     fn add(self, p:Point<T>) -> Point<T> {
//         Point {
//             x: self.x + p.x,
//             y: self.y + p.y,
//         }
//     }
// }
// fn add<T: Add<T,Output = T>>(x:T,y:T) -> T {
//     x+y
// }
// fn main() {
//     let p1 = Point{x: 1.1f32, y: 1.1f32};
//     let p2 = Point{x: 2.1f32, y: 2.1f32};
//     println!("{:?}", add(p1, p2));
//
//     let p3 = Point{x: 1i32, y: 1i32};
//     let p4 = Point{x: 2i32, y: 2i32};
//     println!("{:?}", p3+p4);
// }