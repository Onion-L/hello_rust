#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let width1 = rect1.width;
    let height1 = rect1.height;

    println!("width: {}, height: {}", width1, height1);

    // debug print
    let rect2 = Rectangle {
        width: dbg!(10 * 2),
        height: 40,
    };

    dbg!(&rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}