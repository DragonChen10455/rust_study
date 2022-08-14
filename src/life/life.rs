// fn main(){
//     {
//         let r;
//         {
//             let x = 5;
//             // 借用检查
//             r = &x;
//         }
//         println!("r: {}", r);
//     }
// }

// fn main() {
//     let str1 = "hello";
//     let str2 = "world";
//     let longest_str = longest(str2, str1);
//     println!("{}",longest_str);
//
//     let string1 = "hello";
//     let result;
//     {
//         let string2 = String::from("world");
//         result = longest(string1, string2.as_str());
//     }
//     println!("The longest string is {}",result);
// }
//
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

#[derive(Debug)]
struct Test<'a>{
    temp: &'a str,
}

fn main(){
    let i;
    {
        let hi = "hello world";
        let word = hi.split(" ").next().unwrap();
        i = Test {
            temp: word,
        };
    }
    println!("{:?}",i);

    // 静态生命周期，全局
    let i: &'static str = "hello world";
}