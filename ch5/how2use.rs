//
// fn main() {
//     let mut stack = Vec::new();
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);
//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }
// }

// fn main() {
//     let v=vec![1,2,3,4,5,6,7,8,9,10];
//     // for (index,value) in v.iter().enumerate() {
//     //     println!("{}{}",index,value);
//     //
//     //
//     // }
//     let x= match v.get(0) {
//         Some(1)=> 100,
//         other => 1,
//     };
//
//     println!("{}", x);
//
//
// }

// fn print_coordinates(&(x,y): &(i32,i32)) {
//
//     println!("({},{}): {}", x, y, x+y);
//
//
// }
//
//
// fn main() {
//     let point = (1,2);
//     print_coordinates(&point);
//
// }

// fn main() {
//     let mut state=(0,1);
//     let mut x= ||{
//         state=(state.1,state.0+state.1);
//         state.0
//
//     };
//
//     for _ in 0..20{
//         println!("{}", x());
//     }
// }

// fn main() {
//     let x = Some(5);
//     let y = 10;
//
//     match x {
//         Some(50) => println!("Got 50"),
//         Some(y) => println!("Matched, y = {:?}", y),
//         _ => println!("Default case, x = {:?}", x),
//     }
//
//     println!("at the end: x = {:?}, y = {:?}", x, y);
// }



// fn main() {
    // let x =112312312;
    // match x {
    //     1|2 => println!("one or 2"),
    //     _=> println!("something else"),
    //
    //
    // }

    // let x=5;
    // match x {
    //     1..=5 => println!("small or equal 5"),
    //     other=> println!("Some " ),
    //
    //
    // }
//     let x = 'A';
//
//     match x {
//         'a'..='j' => println!("early ASCII letter"),
//         'k'..='z' => println!("late ASCII letter"),
//         _ => println!("something else"),
//     }
//
//     struct Point {
//         x:i32,
//         y:i32
//
//     }
//
//     let p= Point { x: 10, y: 20 };
//     let Point { x: a, y: b } = p;
//     // println!("{}, {}", a, b);
//
//     let p= Point { x: 1, y: 1 };
//
//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         Point { x: 0, y } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//
//     }
//
//
//
// }
//

// enum Message{
//     Quit,
//     Move{x:i32,y:i32},
//     Write(String),
//     ChangeColor(i32,i32,i32),
// }
//
// fn main(){
//     let msg=Message::Move {x: 100,y: 100};
//     match msg{
//         Message::Quit=>{println!("quit!");}
//         Message::Move{x,y}=>{println!("move {:?},{:?}", x,y);}
//         Message::Write(text)=>{println!("{:?}",text);}
//         Message::ChangeColor(r,g,b)=>{println!("change color {:?},{:?},{:?}",r,g,b);}
//
//
//     }
// }

// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
//
// }
//
// enum Message{
//     Quit,
//     Move{x:i32,y:i32},
//     Write(String),
//     ChangeColor(Color),
// }


// struct Point{
//     x: i32,
//     y: i32,
//
// }
// fn main(){
//     // let msg = Message::ChangeColor(Color::Hsv(1,2,3));
//     // match msg{
//     //     Message::ChangeColor(Color::Rgb(r,g,b))=>{
//     //         println!("ChangeColor rgb: {} {} {}", r, g, b)
//     //     }
//     //
//     //     Message::ChangeColor(Color::Hsv(h,s,v))=>{
//     //         println!("ChangeColor rgb: {} {} {}", h, s, v)
//     //     }
//     //
//     //     other=>{println!("None ")}
//     //
//     // }
//
//     let ((feet,inches),Point { x:x1, y:y1}) = ((3,10),Point { x: 3, y: -10 });
//     println!("feet={} inches={} y={}", feet, inches, y1);
//
// }

// fn main() {
//     let arr : [i32; 2] = [1,2];
//     let [x,y]=arr;
//
//     assert_eq!(x,1);
//     assert_eq!(y,2);
//
// }

fn main() {
    let arr: &[u16] = &[114, 514];

    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }

    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }

    let arr: &[u16] = &[];

    assert!(matches!(arr, [..]));
    assert!(!matches!(arr, [x, ..]));








}