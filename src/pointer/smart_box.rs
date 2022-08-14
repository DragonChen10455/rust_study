fn main(){
    drop_fn();
}

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
