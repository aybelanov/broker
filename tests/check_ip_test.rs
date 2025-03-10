use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use broker::common::helpers::is_private_ip;

// Тесты для IPv4
#[test]
fn test_ipv4_private_10() {
    let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)); // 10.0.0.1
    assert!(is_private_ip(&ip), "10.0.0.1 should be private");
}

#[test]
fn test_ipv4_private_172_16() {
    let ip = IpAddr::V4(Ipv4Addr::new(172, 16, 0, 1)); // 172.16.0.1
    assert!(is_private_ip(&ip), "172.16.0.1 should be private");
}

#[test]
fn test_ipv4_private_172_31() {
    let ip = IpAddr::V4(Ipv4Addr::new(172, 31, 255, 254)); // 172.31.255.254
    assert!(is_private_ip(&ip), "172.31.255.254 should be private");
}

#[test]
fn test_ipv4_private_192_168() {
    let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)); // 192.168.1.1
    assert!(is_private_ip(&ip), "192.168.1.1 should be private");
}

#[test]
fn test_ipv4_private_loopback_127() {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)); // 127.0.0.1
    assert!(is_private_ip(&ip), "127.0.0.1 should be private");
}

#[test]
fn test_ipv4_public() {
    let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)); // 8.8.8.8
    assert!(!is_private_ip(&ip), "8.8.8.8 should not be private");
}

#[test]
fn test_ipv4_172_not_private() {
    let ip = IpAddr::V4(Ipv4Addr::new(172, 32, 0, 1)); // 172.32.0.1 (not in 172.16-31 range)
    assert!(!is_private_ip(&ip), "172.32.0.1 should not be private");
}

#[test]
fn test_ipv4_192_not_private() {
    let ip = IpAddr::V4(Ipv4Addr::new(192, 0, 0, 1)); // 192.0.0.1 (not in 192.168 range)
    assert!(!is_private_ip(&ip), "192.0.0.1 should not be private");
}

// Тесты для IPv6
#[test]
fn test_ipv6_private_ula() {
    let ip = IpAddr::V6(Ipv6Addr::new(0xfc00, 0, 0, 0, 0, 0, 0, 1)); // fc00::1
    assert!(is_private_ip(&ip), "fc00::1 should be private (ULA)");
}

#[test]
fn test_ipv6_private_ula_fd() {
    let ip = IpAddr::V6(Ipv6Addr::new(0xfd00, 0, 0, 0, 0, 0, 0, 1)); // fd00::1
    assert!(is_private_ip(&ip), "fd00::1 should be private (ULA)");
}

#[test]
fn test_ipv6_loopback() {
    let ip = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)); // ::1
    assert!(is_private_ip(&ip), "::1 should be private (loopback)");
}

#[test]
fn test_ipv6_public() {
    let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1)); // 2001:0db8::1
    assert!(!is_private_ip(&ip), "2001:0db8::1 should not be private");
}

#[test]
fn test_ipv6_not_ula() {
    let ip = IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)); // fe80::1 (link-local, not ULA)
    assert!(!is_private_ip(&ip), "fe80::1 should not be private (not ULA)");
}