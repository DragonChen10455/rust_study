fn main(){
    let twenty: f64 = 20f64;
    let twenty_one: f64 = 21 as f64;
    let twenty_two = 22_f64;
    let addition = twenty + twenty_one + twenty_two;
    println!("addition: {:.2}", addition);

    let a = Complex { re: 2.1, im: -1.2 };
    println!("{}", a);

    let s = "中国人";
    let a = &s[0..3];
    println!("{}", a);

    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;

    let one = [1, 2, 3];
    let two: [[u8; 3]; 3] = [one, one, one];
    println!("{}", two[0][1])
}