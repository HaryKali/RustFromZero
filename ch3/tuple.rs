// fn main(){
//     let tup:(i32,f64,u8)=(500,6.4,1);
//
//     let (x,y,z)=tup;
//     let a=tup.2;
//     println!("{} {}",x,a);
//
// }

fn main() {
    let mut s1 = String::from("hello ");
    let s2 = "HI";
    let (res, len) = calculate_length(& mut s1);
    println!("The length of '{}' is {}, res is {}.", s1, len, res);
}

fn calculate_length(s: & mut String) -> (String, usize) {
    let length = s.len();
    s.push_str(" hello world");
    (s.to_string(), length)
}
