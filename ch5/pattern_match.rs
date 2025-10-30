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
#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
}



enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),


}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky!!!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState) =>{
            println!("State quarter from {:?}!", UsState);
            25
        }

    }
}
fn main() {

    let test_coin=Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(test_coin));


}