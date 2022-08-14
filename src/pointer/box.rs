#[derive(Debug, PartialEq)]

// fn main(){
//     let mut x = 10;
//     // 获取x的指针
//     let ptr_x = &mut x as *mut i32;
//     let y = Box::new(20);
//     // 获取y的指针
//     let ptr_y = &*y as *const i32;
//     unsafe {
//         *ptr_x += *ptr_y;
//         println!("y: {}", *ptr_y)
//     }
//     println!("x: {}", x)
// }

struct Point {
    x: f64,
    y: f64,
}

fn main(){
    let box_point = Box::new(Point{
        x: 0.0,
        y: 0.0
    });

    let unboxed_point: Point = *box_point;
    // * = box_point.deref()
    assert_eq!(unboxed_point,Point{x:0.0,y:0.0});

    // 数据拷贝
    let arr  = [0;10];
    let arr1 = arr;
    println!("{:?}", arr.len());
    println!("{:?}",arr1.len());

    // 避免数据拷贝 使用box
    let arr = Box::new([0;10]);
    let arr1 = arr;
    // println!("{:?}",arr.len());
    println!("{:?}", arr1.len());

    // 改变了变量的生命周期，可以设置全局变量
    println!("{}", gen_static_str());
}

fn gen_static_str() -> &'static str{
    let mut s = String::new();
    s.push_str("hello, world");
    Box::leak(s.into_boxed_str())
}
