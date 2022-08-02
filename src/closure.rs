fn main(){
    let x = 4;
    // 闭包可以捕获作用域
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    fn factory() -> Box<dyn Fn(i32) -> i32>{
        let num = 5;
        Box::new(move |x| x + num)
    }
    let f = factory();
    let answer = f(1);
    assert_eq!(6, answer);
}