// #[derive(Debug)]
// enum UiObject {
//     Button,
//     SelectBox,
// }
//
// fn main() {
//     let objects = [
//         UiObject::Button,
//         UiObject::SelectBox
//     ];
//
//     for o in objects {
//         draw(o)
//     }
// }
//
// fn draw(o: UiObject) {
//     println!("{:?}",o);
// }
// trait Draw {
//     fn draw(&self) -> String;
// }
//
// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }
//
// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64: {}", *self)
//     }
// }
//
// fn draw1(x: Box<dyn Draw>) {
//     x.draw();
// }
//
// fn draw2(x: &dyn Draw) {
//     x.draw();
// }
//
// fn main() {
//     let x = 1.1f64;
//     // do_something(&x);
//     let y = 8u8;
//
//     // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
//     // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
//     draw1(Box::new(x));
//     // 基于 y 的值创建一个 Box<u8> 类型的智能指针
//     draw1(Box::new(y));
//     draw2(&x);
//     draw2(&y);
// }

// pub trait Draw {
//     fn draw(&self);
// }
//
//
// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }
//
// impl Draw for Button {
//     fn draw(&self) {
//         // 绘制按钮的代码
//     }
// }
//
// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }
//
// impl Draw for SelectBox {
//     fn draw(&self) {
//         // 绘制SelectBox的代码
//     }
// }
//
//
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
// where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
// fn main() {
//     let screen = Screen {
//         components: vec![
//             Box::new(SelectBox {
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from("Yes"),
//                     String::from("Maybe"),
//                     String::from("No")
//                 ],
//             }),
//             // Box::new(Button {
//             //     width: 50,
//             //     height: 10,
//             //     label: String::from("OK"),
//             // }),
//             Box::new(SelectBox {
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from("Yes"),
//                     String::from("Maybe"),
//                     String::from("No")w
//                 ],
//             }),
//         ],
//     };
//
//     screen.run();
// }
// trait Draw {
//     fn draw(&self) -> Self;
// }
//
// #[derive(Clone)]
// struct Button;
// impl Draw for Button {
//     fn draw(&self) -> Self {
//         return self.clone()
//     }
// }
//
// fn main() {
//     let button = Button;
//     let newb = button.draw();
// }


// trait Bird {
//     fn quack(&self) -> String;
// }
//
// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }
//
// impl Bird for Duck {
//     fn quack(&self) -> String{
//         "duck duck".to_string()
//     }
// }
//
// impl Bird for Swan {
//     fn quack(&self) -> String{
//         "swan swan".to_string()
//     }
// }
//
// fn main() {
//     // 填空
//     let duck = Duck;
//     duck.swim();
//
//     let bird = hatch_a_bird(2);
//     // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
//     // bird.swim();
//     // 但它依然可以叫唤
//     assert_eq!(bird.quack(), "duck duck");
//
//     let bird = hatch_a_bird(1);
//     // 这只鸟儿忘了如何飞翔，因此以下代码会报错
//     // bird.fly();
//     // 但它也可以叫唤
//     assert_eq!(bird.quack(), "swan swan");
//
//     println!("Success!")
// }
//
// //实现以下函数
// fn hatch_a_bird(bird_type: i32) -> Box<dyn Bird> {
//     match bird_type {
//         1=>Box::new(Swan),
//         2=>Box::new(Duck),
//         _=> Box::new(Duck),
//
//     }
// }
