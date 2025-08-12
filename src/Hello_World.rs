fn great_world(){
    let southern_germany = "Grüß Gott!";
    let chinese="你好，世界";
    let english="world hello";
    let regions = [southern_germany,chinese,english];
    for region in regions.iter(){
        println!("{}",&region);
    }

}

fn main() {
    great_world();
}
