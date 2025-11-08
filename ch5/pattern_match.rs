// enum Direction {
//     East,
//     West,
//     North,
//     South
// }
//
// fn main() {
//
//     // let dire = Direction::South;
//     //
//     // match dire {
//     //     Direction::North => println!("we match North"),
//     //     Direction::West|Direction::East => println!("we match west or east"),
//     //     _ => println!("we match South"),
//     //
//     // };
//
//     let a = 3;
//     let b = if a > 0 { println!("here1"); 1 } else { println!("HERE2"); -1 };
// }

// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
//
//
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky!!!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
//
// fn main() {
//
//     value_in_cents(Coin::Penny);
// }

// enum IpAddr {
//     Ipv4,
//     Ipv6,
// }
//
// fn main() {
//     let ip1=IpAddr::Ipv4;
//     let ip2=IpAddr::Ipv6;
//     let ip_str= match ip1 {
//         IpAddr::Ipv4 => "127.0.0.1",
//         IpAddr::Ipv6 => "::1",
//
//     };
//     let ip_str= match ip2 {
//         IpAddr::Ipv4 => "127.0.0.1",
//         IpAddr::Ipv6 => "::1",
//
//     };
//
//     println!("{}", ip_str);
// }


// enum UsState {
//     Alabama,
//     Alaska,
// }
//
//
//
// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
//
//
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky!!!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(UsState) =>{
//             println!("State quarter from {:?}!", UsState);
//             25
//         }
//
//     }
// }
// fn main() {
//
//     let test_coin=Coin::Quarter(UsState::Alaska);
//     println!("{}", value_in_cents(test_coin));
//
//

// enum Action{
//     Say(String),
//     MoveTo(i32,i32),
//     ChangeColor(u16,u16,u16)
//
//
//
//
//
// }
//
// fn main(){
//     let actions = [Action::Say("Hello, World!".to_string()),
//         Action::MoveTo(10,20),
//         Action::ChangeColor(5,6,7),];
//
//     for action in &actions {
//         match action {
//             Action::Say(s) => {println!("{}",s); },
//             Action::MoveTo(x,y) => {println!("{:?}",(x,y));},
//             Action::ChangeColor(r,g,b) => {println!("{:?}",(r,g,b));},
//
//
//         }
//
//
//
//     }
//
//
//
// }
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }
//
// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::North | Direction::South => {
//             println!("South or North");
//         },
//     };
// }

// fn main(){
//
//     let some_u8_value = 0u8;
//     match some_u8_value {
//         1 => println!("one"),
//         3 => println!("three"),
//         5 => println!("five"),
//         7 => println!("seven"),
//         _ => (),
//     }
//
//
// }

// #[derive(Debug)]
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }
//
// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         orther => println!("other direction: {:?}", other),
//     };
// }
// fn main() {
//     let age = Some(30);
//     println!("在匹配前，age是{:?}", age);
//     match age {
//         Some(x) =>  println!("匹配出来的age是{}", x),
//         _ => ()
//     }
//     println!("在匹配后，age是{:?}", age);
// }
// fn main() {
//     let age = Some(30);
//     println!("在匹配前，age是{:?}",age);
//     if let Some(age) = age {
//         println!("匹配出来的age是{}",age);
//     }
//
//     println!("在匹配后，age是{:?}",age);
// }

// fn fib(pair: (i32, i32)) -> (i32, i32) {
//     let (current, next) = pair;
//     (next, current + next)
// }
//
// fn main() {
//     let mut state = (0, 1);
//
//     for i in 0..20 {
//         let current = state.0;
//         state = fib(state);
//         println!("第{}项: {}", i + 1, current);
//     }
// }


// fn main(){
//     let mut state=(0,1);
//     let mut next_fib= ||{
//         state=(state.1,state.0+state.1);
//         state.0
//
//     };
//
//     for i in 0..20{
//         println!("{}", next_fib());
//     }
//
//


// 填空

// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::South|Direction::North  => { // 在这里匹配 South 或 North
//             println!("South or North");
//         },
//         _ => println!("West"),
//     };
// }


// fn main() {
//     let boolean = true;
//
//     // 使用 match 表达式填空，并满足以下条件
//     //
//     // boolean = true => binary = 1
//     // boolean = false => binary = 0
//     let binary = match boolean {
//         true => 1,
//         false => 0,
//     };
//
//     assert_eq!(binary, 1);
// }


// 填空
// enum Message {
//     Quit,
//     Move (i32,i32),
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move(1,3),
//         Message::ChangeColor(255,255,0)
//     ];
//
//     for msg in msgs {
//         show_message(msg)
//     }
// }
//
// fn show_message(msg: Message) {
//     match msg {
//         Message::Move(a,b) => { // 这里匹配 Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         },
//         Message::ChangeColor(r, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         _ => println!("no data in these variants")
//     }
// }


// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];
//
//     // 使用 `matches!` 填空
//     for ab in alphabets {
//         assert!(matches!(ab, 'A'..='Z' | '0'..='9'| 'a'..='z'));
//     }
// }

// enum MyEnum {
//     Foo,
//     Bar
// }
//
// fn main() {
//     let mut count = 0;
//
//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         if matches!(e,MyEnum::Foo) { // 修复错误，只能修改本行代码
//             count += 1;
//         }
//     }
//
//     assert_eq!(count, 2);
// }



// fn main() {
//     let o = Some(7);
//
//     // 移除整个 `match` 语句块，使用 `if let` 替代
//     if let Some(i) = o {
//
//         println!("This is a really long string and `{:?}`", i);
//     }
// }


// 填空
// enum Foo {
//     Bar(u8)
// }
//
// fn main() {
//     let a = Foo::Bar(1);
//
//     if let Foo::Bar(x) = a {
//         println!("foobar 持有的值是: {}", x);
//     }
// }


// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }
//
// fn main() {
//     let a = Foo::Qux(10);
//
//     match a {
//         Foo::Qux(value) => println!("Got a value: {}", value),
//         other=> println!("Other")
//
//     }
// }

// 就地修复错误
// fn main() {
//     let age = Some(30);
//     if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
//         assert_eq!(age, 30);
//     } // 新的 `age` 变量在这里超出作用域
//
//     match age {
//         // `match` 也能实现变量遮蔽
//         Some(x) =>  println!("age 是一个新的变量，它的值是 {}",x),
//         _ => ()
//     }
// }

// fn main() {
//     let mut state=(0,1);
//     let mut res= ||{
//         state=(state.1,state.0+state.1);
//         state.0
//     };
//
//     for _ in 0..20{
//
//         println!("{}", res());
//
//
//   }
// }

