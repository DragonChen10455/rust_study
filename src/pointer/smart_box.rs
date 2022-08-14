// 共享所有权的智能指针： Rc<T>,Arc<T>
// 基于引用计数的智能指针，主要用于资源同时存在多份借用的场景

// Rc<T>: 多个服务共用一个变量作为参数
// Arc<T>: 服务在多线程中运行

use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

fn main(){
    struct Service{
        f: Rc<u8>,
    }
    struct Service2{
        f: Arc<u8>,
    }

    let data: Rc<u8> = Rc::new(1_u8);
    let a: Service = Service{f: data.clone()};
    let b: Service = Service{f: data.clone()};

    let data2: Arc<u8> = Arc::new(1_u8);
    let c: Service2 = Service2{f: data2.clone()};
    let d: Service2 = Service2{f: data2.clone()};

    println!("a.f = {}", a.f);
    println!("b.f = {}", b.f);

    // for _i in 0..3{
    //     let c: Rc<u8> = data.clone();
    //     let _h: JoinHandle<()> = thread::spawn(move || {
    //        println!("f = {}", c);
    //     });
    // }

    println!("c.f = {}", c.f);
    println!("d.f = {}", d.f);

    // 多线程应用
    let mut handlers: Vec<JoinHandle<()>> = Vec::new();
    for _i in 0..3{
        let e: Arc<u8> = data2.clone();
        let handle: JoinHandle<()> = thread::spawn(move || {
            println!("f = {}", e);
        });
        handlers.push(handle);
    }
    for handle in handlers {
        handle.join().unwrap();
    }

    let mut data_val = *data2;
    data_val += 1;
    println!("{}",data_val);
}
