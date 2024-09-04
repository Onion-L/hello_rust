fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    str_change(&mut s2);
    println!("{}",s2);

    let r1 = &s2;
    let r2 = &s2;
    println!("{} {}",r1,r2);

    
    let r3 = &mut s2;

    let r4 = r3.replace("world","Rust");
    println!("{} {}",r3,r4);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn str_change(some_string: &mut String) {
    some_string.push_str(", world!");
}