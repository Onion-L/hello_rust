use std::io;

fn main() {
 let mut x = 5;
 {
    let x = [1,2,3];
    println!("The value of x in the inner scope is: {:?}",x);
 }
 x += 1;
 println!("The value of x is: {x}");

 let spaces = "    ";
let spaces = spaces.len();
//spaces = spaces.len();
 println!("The value of spaces is: {spaces}");

 let a = 1.0;
 let b = 1.2;
 print!("a/b = {}", a/b);

 println!("{}",0.1+0.2);
 let heart_eyed_cat = '😻';
 println!("{}",heart_eyed_cat);

 let tup = (1,false,1.1);
 let(_,_,x) = tup;
 println!("{}",x);
 println!("{}",tup.0);

 let a = [1;4];
 println!("{:?}",a);

 let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
