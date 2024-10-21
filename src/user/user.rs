use std::net::IpAddr;

#[derive(Default)]
struct User {
    nick_name: &str,
    address: IpAddr,
}

impl User {
    pub fn new() -> Self {
        Self {
            name: "",
            address: IpAddr::new(),
        }
    }
}
