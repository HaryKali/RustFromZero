#[derive(Debug)]

// #[derive(Debug)]
// enum PokerSuit {
//     Clubs,
//     Spades,
//     Diamonds,
//     Hearts,
// }
//
// struct PokerCard {
//     suit: PokerSuit,
//     value: u8
// }
//
// fn main() {
//     let c1 = PokerCard {
//         suit: PokerSuit::Clubs,
//         value: 1,
//     };
//     let c2 = PokerCard {
//         suit: PokerSuit::Diamonds,
//         value: 12,
//     };
//
// }


// fn main() {
//     let heart = PokerSuit::Hearts;
//     let diamond = PokerSuit::Diamonds;
//
//     print_suit(heart);
//     print_suit(diamond);
// }
//
// fn print_suit(card: PokerSuit) {
//     // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
//     println!("{:?}",card);
// }

// enum PokerSuit {
//     Clubs,
//     Spades,
//     Diamonds,
//     Hearts,
// }
//
// struct PokerCard {
//     suit: PokerSuit,
//     value: u8
// }
//
// fn main() {
//     let c1 = PokerCard {
//         suit: PokerSuit::Clubs,
//         value: 1,
//     };
//     let c2 = PokerCard {
//         suit: PokerSuit::Diamonds,
//         value: 12,
//     };
// }
// enum PokerCard {
//     Clubs(u8),
//     Spades(u8),
//     Diamonds(u8),
//     Hearts(u8),
// }
//
// fn main() {
//     let c1 = PokerCard::Spades(5);
//     let c2 = PokerCard::Diamonds(13);
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
//
//
// }
//
// fn main(){
//     let m1 = Message::ChangeColor(0, 160, 255);
//     let m2=Message::Move { x: 3, y: 4 };
//     let m3=Message::Quit;
//
//
//
// }

// enum Option<T> {
//     Some(T),
//     None,
// }
//
// fn main(){
//     let some_number = Some(5);
//     let some_string = Some("a string");



// }


// 修复错误
// enum Number {
//     Zero,
//     One,
//     Two,
//     Four,
// }
//
// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
//
// }
//
// // C语言风格的枚举定义
// enum Number2 {
//     Zero = 0,
//     One = 1,
//     Two = 2,
// }
//
//
// fn main() {
//     // 通过 `as` 可以将枚举值强转为整数类型
//     assert_eq!(Number::One as u8, Number1::One as u8);
//     assert_eq!(Number1::One as i32, Number2::One as i32);
//     print!("{}", Number::Four as u8);
//
// }

// 填空
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msg1 = Message::Move{x:1,y:2}; // 使用x = 1, y = 2 来初始化
//     let msg2 = Message::Write("Hello world".to_string()); // 使用 "hello, world!" 来初始化
// }


// 仅填空并修复错误
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msg = Message::Move{x: 1, y: 2};
//     let a=1;
//     let b=1;
//     if let Message::Move{x:1,y:2} = msg {
//         assert_eq!(a, b);
//     } else {
//         panic!("不要让这行代码运行！");
//     }
// }


// 填空，并修复错误
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msgs     = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0),
//     ];
//
//     for msg in msgs {
//         show_message(msg)
//     }
// }
//
// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }

// 填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);
        return
    }

    panic!("NEVER LET THIS RUN！");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}