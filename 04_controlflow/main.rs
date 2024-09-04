fn main() {
    println!("Hello, world!");
    if_statement(10);
    let number = if_condition();
    println!("The value of number is: {number}");
    loop_statement();
    loop_label();
    while_statement();
    for_statement();
    for_range();
}

fn if_statement(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    }else {
        println!("number is not divisible by 4 or 3");
    }
}

fn if_condition() -> i32 {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    number
}

fn loop_statement() {
    let mut count = 0;
    let result = loop {

        if count == 2 {
            println!("count is 2");
            count += 2;
            continue;       
        }else if count == 5 {
            break count * 2;
        }
        count += 1;
    };

    println!("The result is {}",result);
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_statement() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_statement() {
    let a = [10,20,30,40,50];
    for element in a {
        println!("The value is: {}",element);
    }
}

fn for_range() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
