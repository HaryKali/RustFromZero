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