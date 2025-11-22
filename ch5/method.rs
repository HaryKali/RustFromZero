/*struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }


    fn area(&self) -> f64 {

        std::f64::consts::PI * (self.radius * self.radius)
    }


}

fn main(){
    let test_c=Circle::new(10.0,10.0,5.0);
    println!("{}", test_c.area());
}*/

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//
//     fn width(&self) -> u32 {
//         self.width
//     }
// }
//
// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };
//
//     // println!(
//     //     "The area of the rectangle is {} square pixels.",
//     //     rect1.area()
//     // );
//
//     let a =rect1.width();
// }

// mod my {
//     pub struct Rectangle {
//         width: u32,
//         pub height: u32,
//     }
//
//     impl Rectangle {
//         pub fn new(width: u32, height: u32) -> Self {
//             Rectangle { width, height }
//         }
//         pub fn width(&self) -> u32 {
//             return self.width;
//         }
//         pub fn height(&self) -> u32 {
//             return self.height;
//         }
//     }
// }
//
// fn main() {
//     let rect1 = my::Rectangle::new(30, 50);
//
//     println!("{}", rect1.width()); // OK
//     println!("{}", rect1.height()); // OK
//     // println!("{}", rect1.width); // Error - the visibility of field defaults to private
//     println!("{}", rect1.height); // OK
// }
// struct Rectangle{
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle{
//
//     fn can_hold(&self, other: &Rectangle) -> bool{
//         if self.width > other.width || self.height > other.height{
//             true
//         }
//         else {
//             false
//         }
//
//     }
// }
//
//
// fn main(){
//
//     let rect1 =Rectangle{height:1,width:1};
//     let rect2 = Rectangle{height:10,width:15};
//     println!("{}",rect2.can_hold(&rect1));
//
//
// }

// #![allow(unused)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// impl Message {
//     fn call(&self) {
//         match self {
//             Message::Quit => println!("Quit"),
//             Message::Move { x, y } => println!("Move({}, {})", x, y),
//             Message::ChangeColor(r, g, b) => println!("ChangeColor({}, {}, {})", r, g, b),
//             Message::Write(text) => println!("Write(\"{}\")", text),
//
//
//         }
//     }
// }
//
//
// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }
