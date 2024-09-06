struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {

    let mut black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {}, origin: {}", black.0, origin.0);

    // black = origin;
    println!("black: {}, origin: {}", black.0, origin.0);

    let mut user1 = User {
        active: true,
        username: String::from("user1_S"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };
    println!("user1: {}", user1.username);
    user1.active = false;
    println!("user1: {}", user1.active);

    let mut user2 = build_user(String::from("user2_S"), String::from("user2@example.com"));
    println!("user2: {}", user2.username);
    user2.active = true;
    println!("user2: {}", user2.active);
    
    println!("user1: {}", user1.username);

    let mut user3 = User {
        active:false,
        ..user1
    };

    println!("user3: {}", user3.username);
    user3.username = String::from("user3_S");
    println!("user3: {}", user3.username);
    user3.active = true;
    println!("user3: {}", user3.active);
    println!("user1: {}", user1.active);
    // println!("user1: {}", user1.username); 
    // user is moved
}

fn build_user(username: String, email: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 1,
    }
}
