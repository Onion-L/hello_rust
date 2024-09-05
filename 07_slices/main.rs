fn main() {
    let s = String::from("Hello Rust");
    let word = str_slice(&s);
    println!("{}", word);

    let hello = &s[..5];
    let rust = &s[6..10];
    println!("String Slice: {} | {}", hello, rust);

    let s2 = "Hello World";
    let s2_slice = &s2[6..];
    println!("String Slice: {}", s2_slice);
    let full_slice = &s2[..];
    println!("Full Slice: {}", full_slice);

    let word = first_word(&s);
    println!("First Word: {}", word);

    let s = String::from("Hello World Again");
    let second_word = second_word(&s);
    println!("Second Word: {}", second_word);

    let a = [1, -2, 3, 4, 5];
    let slice = array_slice(&a);
    println!("Array Slice: {:?}", slice);
}


fn str_slice(s: &str) -> &str {
    s.split(" ").last().expect("Could not find a space in the string.")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }
    &s[..]
}

fn array_slice(a: &[i32]) -> &[i32] {
   &a[1..3]
}