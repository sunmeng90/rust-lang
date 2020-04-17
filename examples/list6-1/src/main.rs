fn main() {
    enum IpAddrKind { V4, V6 }

    struct IpAddr {
        kind: IpAddrKind,
        address: string,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let lookback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
