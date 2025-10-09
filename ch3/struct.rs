struct User{
    active : bool,
    username: String,
    email:String,
    sign_in_count: u64,

}
fn main(){
    let mut test_user1= User{
        email :String::from("123123123@qq.com"),
        username: String::from("123123123"),
        active: true,
        sign_in_count: 1,


    };
    test_user1.email=String::from("456123123");
    let test_user2:User = build_user("apple@qq.com","1123123123");
}

fn build_user(email:&str, username:&str)->User{
    User{
        email:email.to_string(),
        username:username.to_string(),
        active: true,
        sign_in_count: 1,

    }


}