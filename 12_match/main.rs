#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("The value of the coin is: {}", value);

    let coin = Coin::Quarter(State::Alaska);
    let value = value_in_cents(coin);
    println!("The value of the coin is: {}", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}, none: {:?}", six, none);

    let dice_roll = 1;
    match dice_roll {
        3 => println!("You rolled a 3"),
        7 => println!("You rolled a 7"),
        _ => println!("You rolled a {}", dice_roll),
    }

    match dice_roll {
        3 => println!("You rolled a 3"),
        7 => println!("You rolled a 7"),
        _ =>()
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!",state);
            25
        }
    }
}

