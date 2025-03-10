use std::net::IpAddr;

/// Private IP address verification function (IPv4 и IPv6)
pub fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            if octets[0] == 10 { return true; }
            if octets[0] == 172 && (16..=31).contains(&octets[1]) { return true; }
            if octets[0] == 192 && octets[1] == 168 { return true; }
            if octets[0] == 127 { return true; }
            false
        }
        IpAddr::V6(ipv6) => {
            if ipv6.is_loopback() { return true; }
            if ipv6.segments()[0] & 0xfe00 == 0xfc00 { return true; }
            false
        }
    }
}