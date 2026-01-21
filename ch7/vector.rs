
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

// trait IpAddr {
//     fn display(&self);
// }
//
// struct V4(String);
// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4: {:?}",self.0)
//     }
// }
// struct V6(String);
// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6: {:?}",self.0)
//     }
// }
//
// fn main() {
//     let v: Vec<Box<dyn IpAddr>> = vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::1".to_string())),
//     ];
//
//     for ip in v {
//         ip.display();
//     }
// }

// fn main() {
//     let mut vec = vec![1, 5, 10, 2, 15];
//     vec.sort_unstable();
//     assert_eq!(vec, vec![1, 2, 5, 10, 15]);
//     for num in vec {
//         println!("{}", num);
//     }
//
// }

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u32,
// }
//
// impl Person {
//     fn new(name: String, age: u32) -> Person {
//         Person { name, age }
//     }
// }
//
// fn main() {
//     let mut people = vec![
//         Person::new("Zoe".to_string(), 25),
//         Person::new("Al".to_string(), 60),
//         Person::new("John".to_string(), 1),
//     ];
//     people.sort_unstable_by(|a, b| b.age.cmp(&a.age));
//
//     println!("{:?}", people);
// }


// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
//
//     let v = Vec::from(arr);
//     is_vec(&v);
//
//     let v = vec![1, 2, 3];
//     is_vec(&v);
//
//     // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
//     let v = vec!(1, 2, 3);
//     is_vec(&v);
//
//     // ...在下面的代码中, v 是 Vec<[u8; 3]> , 而不是 Vec<u8>
//     // 使用 Vec::new 和 `for` 来重写下面这段代码
//     let mut v1=Vec::new();
//
//     for num in arr{
//         v1.push(num);
//     }
//     let v2=&v1;
//
//     is_vec(&v1);
//
//     assert_eq!(v, v1);
//
//     println!("Success!")
// }
//
// fn is_vec(v: &Vec<u8>) {}


// 填空
// fn main() {
//     let mut v1 = Vec::from([1, 2, 4]);
//     v1.pop();
//     v1.push(3);
//
//     let mut v2 = Vec::new();
//     v2.extend(vec![1, 2, 3]);
//
//     assert_eq!(v1, v2);
//
//     println!("Success!")
// }