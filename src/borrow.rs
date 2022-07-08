fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5,x);
    assert_eq!(5,*y);

    let mut s = String::from("hello");
    // 同一时刻要么一个可变引用，要么任意多个不可变引用
    let _r1 = &s;
    let _r2 = &s;
    let _r3 = &mut s;
    println!("{}",_r3);
    // println!("{},{}", _r1, _r2)

    let reference_success = sample1();
    // let reference_nothing = sample2();
    println!("{}",reference_success);
    // println!("{}",reference_nothing)
}

fn sample1() -> String {
    let s = String::from("hello");
    s
}
// fn sample2() -> &String {
//     let s = String::from("hello");
//     &s
// }