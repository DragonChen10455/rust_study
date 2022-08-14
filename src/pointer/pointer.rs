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