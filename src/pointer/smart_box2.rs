// 内部可变性的智能指针： Cell<T>,RefCell<T>
// 变量需要多个可变引用

// Cell<T>: 以值传递的方式对内部变量进行操作
// RefCell<T>: 可获得内部变量的可变引用

use std::cell::{Cell, RefCell, RefMut};

fn main(){
    let data: Cell<i32> = Cell::new(1_i32);
    let a: &Cell<i32> = &data;
    let b: &Cell<i32> = &data;

    println!("inner = {}", data.get());
    a.set(2);
    println!("inner = {}", data.get());
    b.set(3);
    println!("inner = {}", data.get());
    a.set(4);
    println!("inner = {}", data.get());

    struct Immutable {
        inner: RefCell<i32>,
    }
    impl Immutable {
        // 此处为共享引用修改值
        fn add(&self){
            let mut f_mut: RefMut<i32> = self.inner.borrow_mut();
            *f_mut += 1;
        }
    }
    let a: Immutable = Immutable {
        inner: RefCell::new(0),
    };
    // 此处为可变引用修改值
    *a.inner.borrow_mut() += 1;
    println!("inner = {}", a.inner.borrow());

    // 可变引用只能有一个
    let f: RefMut<i32> = a.inner.borrow_mut();
    println!("inner = {}", f);
    // 可变引用被取出，没有这一句，下面的a.inner无效
    drop(f);

    println!("inner = {}", a.inner.borrow());
    a.add();
    a.add();
    println!("inner = {}", a.inner.borrow());
}

