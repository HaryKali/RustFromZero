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

// 完成两个 `impl` 语句块
// 不要修改 `main` 中的代码
// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//
//     fn say_something(&self) -> String;
// }
//
// struct Student {}
// impl Hello for Student {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//     fn say_something(&self) -> String {
//         String::from("I'm a good student")
//     }
// }
// struct Teacher {}
// impl Hello for Teacher {
//     fn say_hi(&self) -> String {
//         String::from("Hi, I'm your new teacher")
//     }
//     fn say_something(&self) -> String {
//         String::from("I'm not a bad teacher")
//     }
// }
//
// fn main() {
//     let s = Student {};
//     assert_eq!(s.say_hi(), "hi".to_string());
//     assert_eq!(s.say_something(), "I'm a good student".to_string());
//
//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "Hi, I'm your new teacher".to_string());
//     assert_eq!(t.say_something(), "I'm not a bad teacher".to_string());
//
//     println!("Success!")
// }



// `Centimeters`, 一个元组结构体，可以被比较大小
// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);
//
// // `Inches`, 一个元组结构体可以被打印
// #[derive(Debug)]
// #[derive(PartialEq, PartialOrd)]
// struct Inches(i32);
//
// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;
//
//         Centimeters(inches as f64 * 2.54)
//     }
// }
//
//
// #[derive(PartialEq, PartialOrd)]
// #[derive(Debug)]
// struct Seconds(i32);
//
// fn main() {
//     let _one_second = Seconds(1);
//
//     println!("One second looks like: {:?}", _one_second);
//     let _this_is_true = _one_second == _one_second;
//     let _this_is_false = _one_second > _one_second;
//
//     let foot = Inches(12);
//
//     println!("One foot equals {:?}", foot);
//
//     let meter = Centimeters(100.0);
//
//     let cmp =
//         if foot.to_centimeters() < meter {
//             "smaller"
//         } else {
//             "bigger"
//         };
//
//     println!("One foot is {} than one meter.", cmp);
// }


// use std::ops;
//
// // implement fn multiply to make the code work
// // As mentioned above, `+` needs `T` to implement `std::ops::Add` Trait
// // so, what about `*` ?  You can find the answer here: https://doc.rust-lang.org/core/ops/
// fn multiply<T: ops::Mul<Output = T>>(x: T, y: T) -> T {
//     x * y
// }
//
// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));
// }



// 修复错误，不要修改 `main` 中的代码!
// use std::ops;
//
// struct Foo;
// struct Bar;
// #[derive(PartialEq,Debug)]
// struct FooBar;
// #[derive(PartialEq,Debug)]
// struct BarFoo;
//
// // 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;
//
//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
//
// }
//
// impl ops::Sub<Bar> for Foo {
//     type Output = BarFoo;
//     fn sub(self, _rhs: Bar) -> BarFoo {
//         BarFoo
//     }
// }
//
// fn main() {
//     // 不要修改下面代码
//     // 你需要为 FooBar 派生一些特征来让代码工作
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);
//
//     println!("Success!")
// }


// 实现 `fn summary`
// 修复错误且不要移除任何代码行
// trait Summary {
//     fn summarize(&self) -> String;
//
// }
//
// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }
//
// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
//
//     // fn summary(&self) -> String {
//     //     format!("The author of post {} is {}", self.title, self.author)
//     // }
// }
//
// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }
//
// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
//     // fn summary(&self) -> String {
//     //     format!("{} - {}", self.username, self.content)
//     // }
// }
//
// fn main() {
//     let post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };
//
//     summary(&post);
//     summary(&weibo);
//
//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }
//
// pub fn summary<T: Summary>(item: &T)-> String {
//     item.summarize()
//
//
// }


// struct Sheep {}
// struct Cow {}
//
// trait Animal {
//     fn noise(&self) -> String;
// }
//
// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }
//
// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }
//
// // 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
// // 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
// fn random_animal(random_number: f64) -> impl Animal{
//     if random_number < 0.5 {
//         Sheep {}
//     } else {
//         Sheep {}
//     }
// }
//
// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }
//


// fn main() {
//     assert_eq!(sum(1, 2), 3);
// }
//
// // 通过两种方法使用特征约束来实现 `fn sum`
// fn sum<T:std::ops::Add<Output=T>>(x: T, y: T) -> T {
//     x + y
// }

// 修复代码中的错误
// struct Pair<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }
//
// impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }
//
//
//
// fn main() {
//     let pair = Pair{
//         x: 1,
//         y: 2
//     };
//
//     pair.cmp_display();
// }

// 填空
fn example1() {
    // `T: Trait` 是最常使用的方式
    // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(10), 11);
    assert_eq!(cacher.value(15), 11);
}


fn example2() {
    // 还可以使用 `where` 来约束 T
    struct Cacher<T>
    where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(20), 21);
    assert_eq!(cacher.value(25), 21);
}



fn main() {
    example1();
    example2();

    println!("Success!")
}