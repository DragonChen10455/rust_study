fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(z) => println!("matched, z = {:?}",z),
        _ => println!("Default case, x = {:?}", x)
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}