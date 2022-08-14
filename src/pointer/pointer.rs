struct HasDrop1;
struct HasDrop2;

impl Drop for HasDrop1{
    fn drop(&mut self) {
        println!("HasDrop1");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("HasDrop2");
    }
}

struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("HasTwoDrops");
    }
}

// copy 和 drop 不能同时存在
// #[derive(Copy)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("foo");
    }
}

fn drop_fn(){
    println!("drop_fn start!");

    let _x = HasTwoDrops{
        one: HasDrop1,
        two: HasDrop2,
    };

    let foo = Foo;
    drop(foo);
    // println!("{}" ,foo);
    println!("drop_fn end!")
}

fn compare(){
    let ref t; t = &1;
    let m; m = &t;
    let ref n:i32; n = &1;
    println!("{}, {}, {}",t, m, n);

    let ref a = 2;
    let ref b = &2;
    println!("{}, {}", a, b);

    let r = &1;
    let &a = r;
    let a = *r;
    println!("{}, {}, {}", r, &a, a);
}

fn main(){
    let a = [1,2,3];
    let b = &a;
    println!("{:?}", b);
    println!("{}",b[0]);

    let mut c = vec![1,2,3];
    let d = &mut c;
    d.push(4);
    println!("{:?}", d);

    let e = &42;
    println!("{}", *e);

    compare();
    drop_fn();
}