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

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     // 完成 area 方法，返回矩形 Rectangle 的面积
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
//
// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };
//
//     assert_eq!(rect1.area(), 1500);
// }


// 只填空，不要删除任何代码行!
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     pub fn show_state(&self)  {
//         println!("the current state is {}", self.color);
//     }
// }
// fn main() {
//     let light = TrafficLight{
//         color: "red".to_owned(),
//     };
//     // 不要拿走 `light` 的所有权
//     light.show_state();
//     // 否则下面代码会报错
//     println!("{:?}", light);
// }

// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     // 使用 `Self` 填空
//     pub fn show_state(self:&Self)  {
//         println!("the current state is {}", self.color);
//     }
//
//     // 填空，不要使用 `Self` 或其变体
//     pub fn change_state(&mut self) {
//         self.color = "green".to_string()
//     }
// }
// fn main() {}

// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     // 1. 实现下面的关联函数 `new`,
//     // 2. 该函数返回一个 TrafficLight 实例，包含 `color` "red"
//     // 3. 该函数必须使用 `Self` 作为类型，不能在签名或者函数体中使用 `TrafficLight`
//     pub fn new()->TrafficLight {
//         TrafficLight{
//             color: "red".to_string(),
//         }
//     }
//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }
//
// fn main() {
//     let light = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");
// }


// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// // 使用多个 `impl` 语句块重写下面的代码
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//
//
// }
//
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
// fn main() {}


// #[derive(Debug)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }
//
// // 为 TrafficLightColor 实现所需的方法
// impl TrafficLightColor {
//     fn color(&self) -> &str {
//         match self {
//             TrafficLightColor::Red => {"red"}
//             TrafficLightColor::Yellow => {"yellow"}
//             TrafficLightColor::Green => {"green"}
//         }
//     }
// }
//
// fn main() {
//     let c = TrafficLightColor::Yellow;
//
//     assert_eq!(c.color(), "yellow");
//
//     println!("{:?}",c);
// }