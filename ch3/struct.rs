// struct User{
//     active : bool,
//     username: String,
//     email:String,
//     sign_in_count: u64,
//
// }
// fn main(){
//     let mut test_user1= User{
//         email :String::from("123123123@qq.com"),
//         username: String::from("123123123"),
//         active: true,
//         sign_in_count: 1,
//
//
//     };
//     test_user1.email=String::from("456123123");
//     let test_user2:User = build_user("apple@qq.com","1123123123");
//     // let test_user3:User = User{
//     //     active:test_user2.active,
//     //     username:test_user2.username,
//     //     email:"lol".to_string(),
//     //     sign_in_count :test_user2.sign_in_count,
//     //
//     // };
//     let test_user3:User = User{
//         sign_in_count :test_user2.sign_in_count,
//         ..test_user2
//     };
//     dbg!(test_user2.active);
//     dbg!(test_user2.username);
// }
//
// fn build_user(email:&str, username:&str)->User{
//     User{
//         email:email.to_string(),
//         username:username.to_string(),
//         active: true,
//         sign_in_count: 1,
//
//     }
//
//
// }
// fn main() {
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);
//
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//
//
//
// }
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!("rect1 is {:#?}", rect1);
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}