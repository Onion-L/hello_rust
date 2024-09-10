fn main() {
    let five = Some(5);

    let six = plus_one(five);
    println!("six: {:?}", six);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    if let Some(i) = x {
        println!("i: {}", i);
        Some(i + 1)
    }else {
        None
    }
   
}
