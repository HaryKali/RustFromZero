// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }
// fn longest<'a>(x: & 'a str, y: &'a str) -> & 'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


trait DisplayBody{
    fn display(&self);

}

struct News{
    author:String,
    title:String,
    body:String,
    date:String,
}

struct Post{
    author:String,
    title:String,
    body:String,

}

impl DisplayBody for News{
    fn display(&self){
        println!("News author {}", self.author);
    }
}

impl DisplayBody for Post{
    fn display(&self){
        println!("Post author {}", self.author);
    }
}

fn main(){
    // let mut a=(0,1);
    // let mut fib=||{
    //     (a.0,a.1)=(a.1,a.0+a.1);
    //     a.0
    // };
    // for _ in 1..10{
    //     println!("{:?}",fib());
    // }
    //
    let post = News{author:"a".to_string(),title:"b".to_string(),body:"c".to_string(),date:"20210".to_string()};
    post.display();

}