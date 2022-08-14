#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    pub fn new(x: u32, y: u32) -> Rectangle{
        Rectangle {
            width: x,
            height: y,
        }
    }
    pub fn new2(x: u32, y: u32) -> Self{
        Self{
            width: x,
            height: y,
        }
    }

    pub fn area(&self) -> u32{
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.width
    }
    // self 所有权转移
    // &self 所有权不可变借用
    // &mut self 可变借用
}

impl Rectangle {
    fn width(&self) -> u32{
        self.width
    }

    pub fn set_width(&mut self, x: u32){
        self.width = x
    }

    pub fn take_ownership(self){
        println!("self {:?}", self);
    }
}

enum  Action{
    Add,
    Subtract,
}

impl Action {
    fn run(&self, x: i32, y:i32) -> i32 {
        match self{
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // rect1.take_ownership();
    println!("{}",rect1.width);

    let n = Action::Add;
    println!("{:?}",n.run(2,1));
}