//! Chapter 6
//! Enumとパターンマッチング

enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn __main__() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from(""),
    };

    println!(home.kind);
}

