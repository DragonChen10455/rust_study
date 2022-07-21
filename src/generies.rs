fn add<T: std::ops::Add<Output = T>>(a: T,b: T) -> T {
    a + b
}

struct Point<T,U>{
    x:T,
    y:U,
}

impl <T,U> Point<T,U>{
    fn mix_up<V,W>(self, other: Point<V,W>) -> Point<T,W>{
        Point {
            x: self.x,
            y: other.y
        }
    }
}

// 泛型特化
impl Point <f32, f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main(){
    println!("add i8:{}", add(2i8,3i8));
    println!("add i32:{}", add(20, 30));
    println!("add f64:{}", add(1.23, 1.23));

    let p1 = Point { x:5.0_f32, y: 10.4 };
    println!("result: {:?}", p1.distance_from_origin());
    let p2 = Point {x:"hello", y:'c'};
    let p3 = p1.mix_up(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
