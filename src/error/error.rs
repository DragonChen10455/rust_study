use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::net::IpAddr;

fn main(){
    // panic!("crash and burn");

    // let home: IpAddr = "127.0.0.1".parse().unwrap();
    // let home: IpAddr = "127.0.0.1 hello".parse().expect("wanna be a Ipv4Addr");
    // println!("IpAddr:{}",home);

    let _ = test_write_fn().unwrap();
    let _ = re_write_fn().unwrap();
    let a = test_get_num();
}

fn test_write_fn() -> Result<usize, std::io::Error>{
    let f = File::create("./hello1.txt");
    let mut f = match f{
        Ok(file) => file,
        Err(error) => {
            return Err(error)
        }
    };
    let size = f.write("hello".as_bytes());
    let size = match size {
        Ok(size) => size,
        Err(error) => {
            return Err(error);
        }
    };
    f.flush()?;
    Ok(size)
}

fn re_write_fn() -> Result<usize, Box<dyn Error>> {
    let mut f = File::create("./hello2.txt")?;
    let size = f.write("hello".as_bytes())?;
    f.flush()?;
    Ok(size)
}

fn test_get_num() -> Option<i32>{
    let nums = vec![1,2,3,4,5];
    let n = nums.first()?;
    Some(*n)
}