// **** Enum
#[derive(Debug)]

enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]

struct _IpAddress {
    kind: IpAddressKind,
    address: String,
}
impl _IpAddress {
    fn some_fn() {
        println!("Hello Blockchain!");
    }
}

fn main() {
    // **** Enum
    let localhost = IpAddressKind::V4(127, 0, 0, 1);
    println!("localhost = {:#?}", localhost);

    // **** Option Enum
    let x = 5;
    let y = Some(5);
    let sum = x + y.unwrap_or(0);
    println!("sum = {}", sum);
}
