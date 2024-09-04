fn main() {
    println!("Hello, world!");
    another_function();
    let result = add(2,2);
    println!("The result is {}",result);
    print_labeled_measurement(5,'h');
    function_with_variables();
}

fn another_function() {
    println!("Another function.");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn add(a:i32, b:i32) -> i32 {
    a + b
}

fn function_with_variables() {

    let y = {
        let x = 1;
        x + 1
    };
    println!("The value of y is: {y}");
}