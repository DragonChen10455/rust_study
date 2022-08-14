use std::ops::Add;

pub trait Node {
    fn move_to(&mut self, x: f32, y: f32);
    fn draw(&self);
}

struct EmptyNode {
    x: f32,
    y: f32,
}

impl Node for EmptyNode {
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self) {
        println!("EmptyNode: x={} , y={}", self.x, self.y)
    }
}
struct TestNode {
    x: f32,
    y: f32,
}

impl Node for TestNode {
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self) {
        println!("TestNode: x={} , y={}", self.x, self.y)
    }
}

fn draw1(x: &dyn Node){
    x.draw();
}

// 特征对象大小不固定，但是他的引用类型大小固定
fn draw2(x: Box<dyn Node>){
    x.draw();
}

struct Counter {
    num: u32,
}

impl Iterator for Counter {
    type Item  = u32;
    // 实现next一定要有null的情况，不然一直停不下来了，一直next到溢出
    fn next(&mut self) -> Option<Self::Item>{
        self.num += 1;
        Some(self.num)
    }
}

#[derive(Debug, PartialEq)]

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot{
    fn fly(&self);
    fn work();
}

trait Wizard{
    fn fly(&self);
    fn work();
}

struct Human;

impl Pilot for Human{
    fn fly(&self){
        println!("Pilot")
    }
    fn work(){
        println!("Pilot working")
    }
}

impl Wizard for Human{
    fn fly(&self){
        println!("Wizard")
    }
    fn work(){
        println!("Wizard working")
    }
}

impl Human{
    fn fly(&self) {
        println!("Human")
    }

    fn work(){
        println!("Human working")
    }
}

fn main(){
    // let test = EmptyNode{x: 1_f32, y: 2_f32};
    // test.draw();
    // draw1(&test);
    //
    // let test2 = Box::new(TestNode{x:1.2, y:1.3});
    // draw2(test2);

    // let mut c = Counter{num: 1};
    // while c.num < 10 {
    //     println!("num: {}", c.next().unwrap());
    // }
    // println!("num :{}", c.count());

    // assert_eq!(Point{x: 1, y: 0} + Point{x: 2, y: 3}
    //            , Point{x: 3, y: 3});

    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    <Human as Pilot>::fly(&person);

    Human::work();
    <Human as Pilot>::work();
}