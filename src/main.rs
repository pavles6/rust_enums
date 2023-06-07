fn main() {
    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    // option enum & option values

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let mut sum = 0;

    if let Some(y) = y {
        sum = x + y
    }
}

// enum without associated values

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

// enums with associated values

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
