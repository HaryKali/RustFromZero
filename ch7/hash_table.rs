// // fn main() {
// //     use std::collections::HashMap;
// //
// //     let teams_list = vec![
// //         ("中国队".to_string(), 100),
// //         ("美国队".to_string(), 10),
// //         ("日本队".to_string(), 50),
// //     ];
// //
// //     let mut teams_map = HashMap::new();
// //     for team in &teams_list {
// //         teams_map.insert(&team.0, team.1);
// //     }
// //
// //     println!("{:?}",teams_map)
// // }
//
// // fn main(){
// //     use std::collections::HashMap;
// //
// //     let teams_list = vec![
// //         ("中国队".to_string(), 100),
// //         ("美国队".to_string(), 10),
// //         ("日本队".to_string(), 50),
// //     ];
// //
// //     let teams_map: HashMap<_, _> =teams_list.into_iter().collect();
// //
// //     println!("{:?}",teams_map)
// //
// // }
//
// use std::any::type_name;
//
// fn main() {
//     // use std::collections::HashMap;
//
//     // let name = String::from("Sunface");
//     // let age = 18;
//     //
//     // let mut handsome_boys = HashMap::new();
//     // handsome_boys.insert(&name, &age);
//     //
//     // println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
//     // println!("还有，他的真实年龄远远不止{}岁", age);
//
//
//     // let mut scores = HashMap::new();
//     //
//     // scores.insert(String::from("Blue"), 10);
//     // scores.insert(String::from("Yellow"), 50);
//     //
//     // let team_name = String::from("Blue");
//     // let score: Option<&i32> = scores.get(&team_name);
//     // let score: i32 = scores.get(&team_name).copied().unwrap_or(0);
//     // use std::collections::HashMap;
//     //
//     // let mut table = HashMap::new();
//     // table.insert("bee".to_string(), 1);
//     // table.insert("monkey".to_string(), 2);
//     // table.insert("lion".to_string(), 3);
//     // table.insert("tiger".to_string(), 4);
//     // table.insert("monkey".to_string(), 100);
//     // let monkey=table.get("monkey").copied().unwrap_or(0);
//     // let yellow = table.entry("fish".to_string()).or_insert(1999);
//     //
//     // for (k,v) in &table {
//     //     println!("{} : {}", *k, *v);
//     // }
//     //
//     // table.remove("lion");
//     // for (k,v) in &table {
//     //     println!("{} : {}", *k, *v);
//     // }
//     use std::collections::HashMap;
//
//     let text = "hello world wonderful world";
//
//     let mut map = HashMap::new();
//     // 根据空格来切分字符串(英文单词都是通过空格切分)
//     for word in text.split_whitespace() {
//         let mut count = map.entry(word).or_insert(0);
//         *count += 1;
//     }
//
//     println!("{:?}", map);
//
//
// }
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn draw1(x: Box<dyn Draw>) {
    x.draw();
}

fn draw2(x: &dyn Draw) {
    x.draw();
}

fn main() {
    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    draw1(Box::new(x));
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
}