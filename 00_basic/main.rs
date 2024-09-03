mod module;

fn main() {
    println!("Hello, world!");
    let x = 5;
    let mut y = 10;
    y += 5;
    println!("x = {}, y = {}", x, y);
   let num =  add(8,2);
    println!("num = {}",num / 5);
    if num % 2 == 0 {
        println!("num is even");
    } else {
        println!("num is odd");
    }

    let mut count = 0;
    loop {
        count += 1;
        if count == 10 {
            break;
        }
        println!("count = {}",count);
    }

    for number in 1..4 {
        println!("number = {}",number);
    }
    module::greet("John");
    string_slice("Hello, world!");
        // 整数类型
        let a: i32 = 10;
        let b: u32 = 20;
    
        // 浮点数类型
        let c: f32 = 3.14;
        let d: f64 = 2.718;
    
        // 布尔类型
        let e: bool = true;
    
        // 字符类型
        let f: char = 'R';
    
        // 字符串类型
        let g: &str = "Hello, world!";
        let h: String = String::from("Hello, Rust!");
    
        // 数组
        let i: [i32; 4] = [1, 2, 3, 4];
    
        // 元组
        let j: (i32, f64, bool) = (42, 3.14, false);
    
        // Option
        let k: Option<i32> = Some(5);
        let l: Option<i32> = None;
    
        // Result
        let m: Result<i32, String> = Ok(10);
        let n: Result<i32, String> = Err("Something went wrong".to_string());
    
        // 打印所有变量
        println!("a: {}, b: {}", a, b);
        println!("c: {}, d: {}", c, d);
        println!("e: {}", e);
        println!("f: {}", f);
        println!("g: {}, h: {}", g, h);
        println!("i: {:?}", i);
        println!("j: {:?}", j);
        println!("k: {:?}, l: {:?}", k, l);
        println!("m: {:?}, n: {:?}", m, n);
}

fn add(a: i32, b: i32) -> i32 {
    return a+b
}

fn string_slice(s: &str) {
    println!("s = {}",s);
}