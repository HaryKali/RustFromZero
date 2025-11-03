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

fn fib(pair: (i32, i32)) -> (i32, i32) {
    let (current, next) = pair;
    (next, current + next)
}

fn main() {
    let mut state = (0, 1);

    for i in 0..20 {
        let current = state.0;
        state = fib(state);
        println!("第{}项: {}", i + 1, current);
    }
}
