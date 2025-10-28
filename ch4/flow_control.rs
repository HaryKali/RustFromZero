// fn main() {
//
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//
//     println!("The number is {}", number);
//
// }

// fn main() {
//     let n = 6;
//
//     if n % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if n % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if n % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }


// fn main() {
//     for i in 1..=5 {
//         println!("{}",i)
//
//     }
//
// }


// fn main() {
    // let a = [4,3,2,1];
    //
    // for (i,v) in a.iter().enumerate() {
    //     println!("{} {}",i,v);
    // }
    // //
    // let mut count = 0;
    // for _ in 0..10{
    //     count += 1;
    //     println!("{}",count);
    //
    // }
    // let collection = [1, 2, 3, 4, 5];
    // for item in collection.iter() {
    //
    //     println!("{}", item);
    // }
    // for i in 1..4 {
    //     if i == 2 {
    //         continue;
    //     }
    //     println!("{}", i);
    // }
//     for i in 1..4 {
//         if i == 2 {
//             break;
//         }
//         println!("{}", i);
//     }
//
//
// }
// fn main() {
//     let mut n = 0;
//
//     loop {
//         if n > 5 {
//             break
//         }
//         println!("{}", n);
//         n+=1;
//     }
//
//     println!("我出来了！");
// }
// fn main() {
//     let mut n = 0;
//
//     while n <= 5  {
//         println!("{}!", n);
//
//         n = n + 1;
//     }
//
//     println!("我出来了！");
// }

fn main() {
    let mut counter =0;
    let result = loop {
        counter+=1;
        if counter ==10{
            break counter*2;
        }
    };

    println!("The result is {}", result);


}