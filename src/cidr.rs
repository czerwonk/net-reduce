use std::net::{Ipv4Addr, Ipv6Addr};

use ipnet::{IpNet, Ipv4Net, Ipv6Net};

/// Parses a stringinto an `IpNet`.
///
/// This function can parse a string that is either in CIDR notation
/// (e.g., "10.1.1.0/24" or "2001:db8::/32"), or a single IP address
/// (e.g., "10.1.1.1" or "2001:db8::1").
///
/// When a single IP address is provided, it is converted into a network
/// with a full-length prefix (/32 for IPv4 and /128 for IPv6).
///
/// The string is trimmed of leading and trailing whitespace before parsing.
///
pub fn from_str(s: &str) -> Option<IpNet> {
    let s = s.trim();

    if let Ok(ip) = s.parse::<IpNet>() {
        return Some(ip);
    }

    if s.contains(":") {
        if let Ok(ip) = s.parse::<Ipv6Addr>() {
            return ipv6_to_ipnet(ip);
        }
    }

    if let Ok(ip) = s.parse::<Ipv4Addr>() {
        return ipv4_to_ipnet(ip);
    }

    None
}

fn ipv4_to_ipnet(ip: Ipv4Addr) -> Option<IpNet> {
    match Ipv4Net::new(ip, 32) {
        Ok(net) => Some(IpNet::V4(net)),
        Err(_) => None,
    }
}

fn ipv6_to_ipnet(ip: Ipv6Addr) -> Option<IpNet> {
    match Ipv6Net::new(ip, 128) {
        Ok(net) => Some(IpNet::V6(net)),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ipnet::{IpNet, Ipv4Net, Ipv6Net};
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_parse_to_cidr_with_cidr_notation() {
        let net: Ipv4Net = "192.168.0.0/24".parse().unwrap();
        assert_eq!(from_str(" 192.168.0.0/24 "), Some(IpNet::V4(net)));
    }

    #[test]
    fn test_parse_to_cidr_with_single_ipv4_address() {
        let ip = Ipv4Addr::new(10, 0, 0, 1);
        let net = Ipv4Net::new(ip, 32).unwrap();
        assert_eq!(from_str("10.0.0.1"), Some(IpNet::V4(net)));
    }

    #[test]
    fn test_parse_to_cidr_with_single_ipv6_address() {
        let ip = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let net = Ipv6Net::new(ip, 128).unwrap();
        assert_eq!(from_str("2001:db8::1"), Some(IpNet::V6(net)));
    }

    #[test]
    fn test_parse_to_cidr_with_invalid_input() {
        assert_eq!(from_str("not an ip"), None);
    }

    #[test]
    fn test_parse_to_cidr_with_whitespace() {
        let ip = Ipv4Addr::new(10, 0, 0, 1);
        let net = Ipv4Net::new(ip, 32).unwrap();
        assert_eq!(from_str("  10.0.0.1  "), Some(IpNet::V4(net)));
    }
}
