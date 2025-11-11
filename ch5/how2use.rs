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



fn main() {
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
    let x = 'A';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x:i32,
        y:i32

    }

    let p= Point { x: 10, y: 20 };
    let Point { x: a, y: b } = p;
    // println!("{}, {}", a, b);

    let p= Point { x: 1, y: 1 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),

    }



}

