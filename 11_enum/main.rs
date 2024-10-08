#[derive(Debug)]
enum IpAddrKind {
    v4(u8, u8, u8, u8),
    v6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}



fn main() {
    let home = IpAddr {
        kind: IpAddrKind::v4(127,0,0,1),
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::v6(String::from("::1")),
        address: String::from("::1"),
    };
    dbg!(&home);


    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
}