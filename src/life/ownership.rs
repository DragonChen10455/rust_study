fn main() {
    let s1 = give_ownership();
    take_ownership(s1);
    // print!("s1={}",s1);
    let s2 = String::from("hello");
    let s3 = take_and_give_back(s2);
    // println!("s2={}",s2);
    println!("s3={}",s3);
}

fn give_ownership() -> String{
    let some_string = String::from("hello");
    some_string
}

fn take_ownership(some_string: String){
    println!("{}",some_string);
}

fn take_and_give_back(mut str: String) -> String{
    str.push_str(",world");
    str
}