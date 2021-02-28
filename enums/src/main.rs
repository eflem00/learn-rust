enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    route(&home);
    route(&loopback);

    let some_string = Some("a string");
    // let some_number = Some(5);
    // let absent_number: Option<i32> = None;

    if some_string.is_some() {
        println!("{}", some_string.unwrap())
    }
}

fn route(ip_addr: &IpAddr) {
    match ip_addr {
        IpAddr::V4(v1, v2, v3, v4) => println!("v4 addr detected {}.{}.{}.{}", v1, v2, v3, v4),
        IpAddr::V6(v) => println!("v6 addr detected {}", v)
    }
}

