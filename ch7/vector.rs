
// fn main(){
//     // let v:Vec<i32>=vec![1,2,3,4,5];
//     // let mut v2:Vec<_>=Vec::new();
//     // v2.push(1);
//     // let mut v3=vec![1,2,3,4];
//     // for num in v3.iter(){
//     //     println!("{}",num)
//     // }
//     // let v = vec![1, 2];
//     //
//     // // let third: &i32 = &v[2];
//     //
//     // match v.get(2){
//     //     Some(third) => println!("{}",third),
//     //     None => println!("None"),
//     // }
//     // let v = vec![1, 2, 3, 4, 5];
//     //
//     // // let does_not_exist = &v[100];
//     // let does_not_exist = v.get(100); // This is better
//     // println!("{:?}", does_not_exist);
//     // let mut v = vec![1, 2, 3, 4, 5];
//     //
//     // let first = &v[0];
//     // println!("The first element is: {first}");
//     // v.push(6);
//     // let v = vec![1, 2, 3];
//     // for i in &v {
//     //     println!("{i}");
//     // }
//     // let mut v = vec![1, 2, 3];
//     // for i in &mut v {
//     //     *i += 10
//     // }
//
//
//
// }

trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

fn main() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}