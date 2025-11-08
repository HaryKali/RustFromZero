
fn plus_one(x:Option<i32>) -> Option<i32>{
    let res=match x {
        None=>None,
        Some(i)=>Some(i+1)


    };
    res
}

fn main() {
    let res=plus_one(Some(5));
    println!("{:#?}",res);
}