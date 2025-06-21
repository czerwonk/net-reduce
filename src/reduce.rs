use std::net::{Ipv4Addr, Ipv6Addr};

use ipnet::{IpNet, Ipv4Net, Ipv6Net};

use crate::reduce_trie::ReduceTrie;

/// Reduces a list of CIDR notations and IP addresses by removing redundant entries.
///
/// This function takes a collection of IP addresses and CIDR blocks (both IPv4 and IPv6)
/// and returns a minimal set where more specific entries that are already covered by
/// broader CIDR blocks are removed.
///
/// # Arguments
///
/// * `lines` - A vector of strings containing IP addresses and/or CIDR notations. Invalid entries are silently ignored.
///
/// # Returns
///
/// A vector of strings containing the reduced set of CIDR notations. All entries
/// are returned in CIDR format (individual IPs are converted to /32 or /128).
///
/// # Examples
///
/// ```
/// use net_reduce::reduce::reduce_cidrs;
///
/// let input = vec![
///     "192.168.0.0/16".to_string(),
///     "192.168.1.0/24".to_string(),  // Covered by /16
///     "192.168.1.1".to_string(),      // Covered by /16
///     "10.0.0.0/8".to_string(),
/// ];
///
/// let result = reduce_cidrs(input);
/// assert_eq!(result.len(), 2);  // Only /16 and /8 remain
/// ```
pub fn reduce_cidrs(lines: Vec<String>) -> Vec<String> {
    let prefixes = lines
        .iter()
        .filter_map(|line| parse_to_cidr(line))
        .collect::<Vec<IpNet>>();

    ReduceTrie::with_prefixes(prefixes)
        .get_all_prefixes()
        .iter()
        .map(|p| p.to_string())
        .collect()
}

fn parse_to_cidr(s: &str) -> Option<IpNet> {
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

    #[test]
    fn test_reduce_cidrs() {
        let lines = vec![
            "2001:678:1e0:0::/64".to_string(),
            "2001:678:1e0::1".to_string(),
            "2001:678:1e0:100::/56".to_string(),
            "2001:678:1e0:110::1/128".to_string(),
            "2001:678:1e0:200::2/128".to_string(),
            "2001:678:1e0:2xx::2/128".to_string(),
            "172.24.0.1".to_string(),
            "192.168.2.0/24".to_string(),
            "192.168.0.0/16".to_string(),
            "192,45.3.1".to_string(),
        ];

        let mut expected = vec![
            "172.24.0.1/32".to_string(),
            "192.168.0.0/16".to_string(),
            "2001:678:1e0::/64".to_string(),
            "2001:678:1e0:100::/56".to_string(),
            "2001:678:1e0:200::2/128".to_string(),
        ];
        expected.sort();

        let mut result = reduce_cidrs(lines);
        result.sort();

        assert_eq!(expected, result);
    }
}
