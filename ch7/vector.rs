
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


// 填空
// fn main() {
//     // array -> Vec
//     // impl From<[T; N]> for Vec
//     let arr = [1, 2, 3];
//     let v1 = Vec::from(arr);
//     let v2: Vec<i32> = arr.to_vec();
//
//     assert_eq!(v1, v2);
//
//
//     // String -> Vec
//     // impl From<String> for Vec
//     let s = "hello".to_string();
//     let v1: Vec<u8> = s.into_bytes();
//
//     let s = "hello".to_string();
//     let v2 = s.into_bytes();
//     assert_eq!(v1, v2);
//
//     // impl<'_> From<&'_ str> for Vec
//     let s = "hello";
//     let v3 = Vec::from(s);
//     assert_eq!(v2, v3);
//
//     // 迭代器 Iterators 可以通过 collect 变成 Vec
//     println!("Success!")
// }


// 修复错误并实现缺失的代码
// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..5 {
//         println!("{:?}", v.get(i))
//     }
//
//     for i in 0..5 {
//         if let Some(x) = v.get(i) {
//             v[i] = x + 1
//         } else {
//             v.push(i + 2)
//         }
//     }
//
//     assert_eq!(format!("{:?}",v), format!("{:?}", vec![2, 3, 4, 5, 6]));
//
//     println!("Success!")
// }


// 修复错误
// fn main() {
//     let mut v = vec![1, 2, 3];
//
//     let slice1 = &v[..];
//     // 越界访问将导致 panic.
//     // 修改时必须使用 `v.len`
//     let slice2 = &v[0..v.len()];
//
//     assert_eq!(slice1, slice2);
//
//     // 切片是只读的
//     // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
//     let vec_ref: &mut Vec<i32> = &mut v;
//     (*vec_ref).push(4);
//     let slice3 = &mut v[0..3];
//
//
//     assert_eq!(slice3, &[1, 2, 3,]);
//
//     println!("Success!")
// }


// 修复错误

// 修复错误
// fn main() {
//     let mut vec = Vec::with_capacity(10);
//
//     assert_eq!(vec.len(), 0);
//     assert_eq!(vec.capacity(), 10);
//
//     // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
//     for i in 0..10 {
//         vec.push(i);
//     }
//     assert_eq!(vec.len(), 10);
//     assert_eq!(vec.capacity(), 10);
//
//     // ...但是下面的代码会造成新的内存分配
//     vec.push(11);
//     assert_eq!(vec.len(), 11);
//     assert!(vec.capacity() >= 11);
//
//
//     // 填写一个合适的值，在 `for` 循环运行的过程中，不会造成任何内存分配
//     let mut vec = Vec::with_capacity(100);
//     for i in 0..100 {
//         vec.push(i);
//     }
//
//     assert_eq!(vec.len(), 100);
//     assert_eq!(vec.capacity(), 100);
//
//     println!("Success!")
// }

// #[derive(Debug)]
// #[derive(PartialEq)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     // 填空
//     let v : Vec<IpAddr>= vec![IpAddr::V4("127.0.0.1".to_string()),
//                               IpAddr::V6("::1".to_string())
//     ];
//
//     // 枚举的比较需要派生 PartialEq 特征
//     assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
//     assert_eq!(v[1], IpAddr::V6("::1".to_string()));
//
//     println!("Success!")
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
//     // 填空
//     let v: Vec<Box<IpAddr>>= vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::1".to_string())),
//     ];
//
//     for ip in v {
//         ip.display();
//     }
// }