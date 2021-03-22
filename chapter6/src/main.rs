#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("Enums");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("V4: {:?}", four);
    println!("V6: {:?}", six);
}
