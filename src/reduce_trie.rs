use std::collections::HashMap;

use ipnet::IpNet;

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 2],
    prefix: Option<IpNet>,
}

struct Table {
    root: Node,
    hosts: Vec<IpNet>,
}

/// A trie structure to reduce IP prefixes. The trie only stores the less specific prefixes for
/// each IPv4 or IPv6 address.
pub struct ReduceTrie {
    ipv4: Table,
    ipv6: Table,
}

impl ReduceTrie {
    /// Creates a new `ReduceTrie` with the given prefixes.
    pub fn with_prefixes(prefixes: Vec<IpNet>) -> Self {
        let (ipv4_prefixes, ipv6_prefixes): (Vec<_>, Vec<_>) = prefixes
            .into_iter()
            .partition(|p| matches!(p, IpNet::V4(_)));

        let (ipv4, ipv6) = rayon::join(
            || Self::build_for_family(ipv4_prefixes),
            || Self::build_for_family(ipv6_prefixes),
        );

        ReduceTrie { ipv4, ipv6 }
    }

    fn build_for_family(prefixes: Vec<IpNet>) -> Table {
        let mut root = Node::default();

        let sorted_prefixes = sort_prefixes(prefixes);

        let (net_prefixes, host_prefixes): (Vec<_>, Vec<_>) = sorted_prefixes
            .into_iter()
            .partition(|p| p.prefix_len() < p.max_prefix_len());

        for prefix in net_prefixes {
            Self::insert_into_node(&mut root, prefix);
        }

        let mut hosts = Vec::new();
        for host in host_prefixes {
            if Self::find_insert_leaf(&mut root, host).is_some() {
                hosts.push(host);
            }
        }

        Table { root, hosts }
    }

    fn insert_into_node(root: &mut Node, prefix: IpNet) -> bool {
        let prefix_len = prefix.prefix_len() as usize;
        let mut node = root;

        let start_pos;
        match Self::find_insert_leaf(node, prefix) {
            Some((n, pos)) => {
                start_pos = pos as usize;
                node = n;
            }
            None => {
                // the prefix is already covered
                return false;
            }
        };

        for pos in start_pos..prefix_len {
            let bit = get_bit(&prefix, pos) as usize;

            if node.children[bit].is_none() {
                node.children[bit] = Some(Box::default());
            }
            node = node.children[bit].as_mut().unwrap();
        }

        node.prefix = Some(prefix);
        node.children[0] = None;
        node.children[1] = None;

        true
    }

    fn find_insert_leaf(root: &mut Node, prefix: IpNet) -> Option<(&mut Node, u8)> {
        let prefix_len = prefix.prefix_len() as usize;
        let mut node = root;

        for pos in 0..prefix_len {
            let bit = get_bit(&prefix, pos) as usize;

            if node.prefix.is_some() {
                // the prefix is alredy covered
                return None;
            }

            if node.children[bit].is_none() {
                return Some((node, pos as u8));
            }
            node = node.children[bit].as_mut().unwrap();
        }

        Some((node, prefix_len as u8))
    }

    pub fn get_all_prefixes(&self) -> Vec<IpNet> {
        let mut result = Vec::new();

        collect_prefixes(&self.ipv4.root, &mut result);
        collect_prefixes(&self.ipv6.root, &mut result);
        result.extend(self.ipv4.hosts.iter());
        result.extend(self.ipv6.hosts.iter());

        result
    }
}

fn get_bit(prefix: &IpNet, pos: usize) -> u8 {
    let byte_idx = pos >> 3; // divide by 8
    let bit_idx = 7 - (pos & 7); // modulo 8

    match prefix {
        IpNet::V4(net) => {
            let bytes = net.addr().octets();
            (bytes[byte_idx] >> bit_idx) & 1
        }
        IpNet::V6(net) => {
            let bytes = net.addr().octets();
            (bytes[byte_idx] >> bit_idx) & 1
        }
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

fn collect_prefixes(node: &Node, result: &mut Vec<IpNet>) {
    if let Some(prefix) = &node.prefix {
        result.push(*prefix);
        // don't traverse children of nodes with prefixes
        return;
    }

    if let Some(child) = &node.children[0] {
        collect_prefixes(child, result);
    }
    if let Some(child) = &node.children[1] {
        collect_prefixes(child, result);
    }
}
