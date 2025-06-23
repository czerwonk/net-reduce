use std::net::{Ipv4Addr, Ipv6Addr};

use ipnet::{IpNet, Ipv4Net, Ipv6Net};

/// Parses a string into an `IpNet`.
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
/// # Examples
///
/// ```
/// use std::net::{Ipv4Addr, Ipv6Addr};
/// use ipnet::{IpNet, Ipv4Net, Ipv6Net};
/// use net_reduce::parse_to_cidr;
///
/// // CIDR notation
/// let net: Ipv4Net = "192.168.0.0/24".parse().unwrap();
/// assert_eq!(parse_to_cidr(" 192.168.0.0/24 "), Some(IpNet::V4(net)));
///
/// // Single IPv4 address
/// let ip = Ipv4Addr::new(10, 0, 0, 1);
/// let net = Ipv4Net::new(ip, 32).unwrap();
/// assert_eq!(parse_to_cidr("10.0.0.1"), Some(IpNet::V4(net)));
///
/// // Single IPv6 address
/// let ip = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
/// let net = Ipv6Net::new(ip, 128).unwrap();
/// assert_eq!(parse_to_cidr("2001:db8::1"), Some(IpNet::V6(net)));
///
/// // Invalid input
/// assert_eq!(parse_to_cidr("not an ip"), None);
/// ```
pub fn parse_to_cidr(s: &str) -> Option<IpNet> {
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
