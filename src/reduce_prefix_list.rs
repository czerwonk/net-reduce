use ipnet::IpNet;
use std::collections::HashMap;

/// A sorted list of IP prefixes that uses binary search for efficient lookups
pub struct ReducePrefixList {
    ipv4_prefixes: Vec<IpNet>,
    ipv6_prefixes: Vec<IpNet>,
}

impl ReducePrefixList {
    pub fn new() -> Self {
        ReducePrefixList {
            ipv4_prefixes: Vec::new(),
            ipv6_prefixes: Vec::new(),
        }
    }

    pub fn with_prefixes(prefixes: Vec<IpNet>) -> Self {
        let mut list = ReducePrefixList::new();

        for prefix in sort_prefixes(prefixes) {
            list.insert(prefix);
        }

        list
    }

    fn insert(&mut self, prefix: IpNet) -> bool {
        let prefixes = match prefix {
            IpNet::V4(_) => &mut self.ipv4_prefixes,
            IpNet::V6(_) => &mut self.ipv6_prefixes,
        };

        for existing in prefixes.iter() {
            if existing.contains(&prefix.network()) {
                return false; // Already covered
            }
        }

        prefixes.push(prefix);
        true
    }

    pub fn get_all_prefixes(&self) -> Vec<IpNet> {
        let mut result = Vec::with_capacity(self.ipv4_prefixes.len() + self.ipv6_prefixes.len());
        result.extend(&self.ipv4_prefixes);
        result.extend(&self.ipv6_prefixes);
        result
    }
}

fn sort_prefixes(prefixes: Vec<IpNet>) -> Vec<IpNet> {
    // reasoning: we use the grouping approach here, since it is very expensive to sort for prefix length on
    // large sets of prefixes. in this case we can sort by iterating over the preixes (O(n)) and
    // then sorting the keys (O(m log m)), where m is the number of unique prefix lengths.

    let mut grouped_prefixes: HashMap<u8, Vec<IpNet>> = HashMap::new();

    for p in prefixes {
        grouped_prefixes.entry(p.prefix_len()).or_default().push(p);
    }

    let mut keys: Vec<u8> = grouped_prefixes.keys().copied().collect();
    keys.sort_unstable();

    let mut result = Vec::new();
    for k in keys {
        result.extend(grouped_prefixes.remove(&k).unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_reduce() {
        let mut trie = ReducePrefixList::new();

        assert!(trie.insert(IpNet::from_str("10.0.0.0/8").expect("valid prefix")));
        assert!(trie.insert(IpNet::from_str("fd00::/8").expect("valid prefix")));
        assert!(trie.insert(IpNet::from_str("192.168.0.0/16").expect("valid prtest_reduceefix")));
        assert!(!trie.insert(IpNet::from_str("10.0.1.0/24").expect("valid prefix")));
        assert!(trie.insert(IpNet::from_str("2001:678:1e0::/48").expect("valid prefix")));
        assert!(!trie.insert(IpNet::from_str("2001:678:1e0:100::/56").expect("valid prefix")));
    }
}
