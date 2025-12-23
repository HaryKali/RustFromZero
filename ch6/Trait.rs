// pub trait Summary {
//     fn summarize(&self) -> String;
// }
pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

pub struct Weibo{
    pub content: String,
    pub username: String,
}
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
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

