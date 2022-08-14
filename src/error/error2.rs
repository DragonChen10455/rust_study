// use std::{io, num};
// use std::fs::File;
// use std::io::Read;
//
// // 自定义错误
// #[derive(Debug)]
// struct AppError {
//     kind: String,
//     message: String,
// }
//
// impl From<io::Error> for AppError {
//     fn from(error:io::Error) -> Self {
//         AppError{
//             kind:String::from("io"),
//             message: error.to_string(),
//         }
//     }
// }
//
// impl From<num::ParseIntError> for AppError{
//     fn from(error: num::ParseIntError) -> Self {
//         AppError{
//             kind:String::from("parse"),
//             message:error.to_string(),
//         }
//     }
// }
//
// fn main() -> Result<(), AppError>{
//     let mut file = File::open("./test.txt")?;
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;
//     content = content.trim().parse().unwrap();
//     let _number: usize;
//     _number = content.parse()?;
//
//     Ok(())
// }

fn main(){
    let s1 = Some("abcde");
    let s2 = Some(5);
    let trans_fn = |s: &str| s.chars().count();
    // map可以将Some或Ok中的值映射到另一个
    assert_eq!(s1.map(trans_fn), s2);

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);
    assert_eq!(o1.map(trans_fn), o2);

    // map_err对Err中的值进行改变
    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);
    let trans_fn2 = |s: &str| -> isize { s.parse().unwrap() };
    assert_eq!(e1.map_err(trans_fn2), e2);
}

