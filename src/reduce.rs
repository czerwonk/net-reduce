use std::net::{Ipv4Addr, Ipv6Addr};

use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use prefix_trie::joint::map::JointPrefixMap;

pub fn reduce_cidrs(lines: Vec<String>) -> Vec<String> {
    build_trie(lines)
        .iter()
        .map(|(prefix, _)| prefix.to_string())
        .collect()
}

fn build_trie(lines: Vec<String>) -> JointPrefixMap<IpNet, bool> {
    let mut pm: JointPrefixMap<IpNet, bool> = JointPrefixMap::new();

    let mut prefixes: Vec<IpNet> = lines
        .iter()
        .filter_map(|line| parse_to_cidr(line))
        .collect();
    prefixes.sort_by_key(|p| p.prefix_len());

    for prefix in prefixes {
        if pm.get_spm_prefix(&prefix).is_some() {
            continue;
        }

        pm.insert(prefix, true);
    }

    pm
}

fn parse_to_cidr(s: &str) -> Option<IpNet> {
    let s = s.trim();

    if let Ok(ip) = s.parse::<IpNet>() {
        return Some(ip);
    }

    if s.contains(":") {
        if let Ok(ip) = s.parse::<Ipv6Addr>() {
            return Some(IpNet::V6(Ipv6Net::new(ip, 128).unwrap()));
        }
    }

    if let Ok(ip) = s.parse::<Ipv4Addr>() {
        return Some(IpNet::V4(Ipv4Net::new(ip, 32).unwrap()));
    }

    None
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
            "172.24.0.1".to_string(),
            "192.168.2.0/24".to_string(),
            "192.168.0.0/16".to_string(),
        ];

        let expected = vec![
            "172.24.0.1/32".to_string(),
            "192.168.0.0/16".to_string(),
            "2001:678:1e0::/64".to_string(),
            "2001:678:1e0:100::/56".to_string(),
            "2001:678:1e0:200::2/128".to_string(),
        ];

        let result = reduce_cidrs(lines);
        assert_eq!(expected, result);
    }
}
