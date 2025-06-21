# net-reduce
Simple tool for reducing (removing more specifics) CIDR/IP addresses from standard input

This is a rewrite of [net-merge](https://github.com/czerwonk/net-merge) in rust.

## Remarks

Since the initial use case is preparing data to feed in a firewall the internal data structure is optimized for host prefixes.
First a trie is built for all net prefixes (parallel for IPv4 and IPv6), then coverage of the host prefixes is checked in parallel.

## Dependencies

This project uses the following Rust crates:

- **[anyhow](https://crates.io/crates/anyhow)** - Flexible error handling library
- **[clap](https://crates.io/crates/clap)** - Command line argument parser
- **[ipnet](https://crates.io/crates/ipnet)** - IP network address manipulation
- **[rayon](https://crates.io/crates/rayon)** - Parallel processing

## License
(c) Daniel Brendgen-Czerwonk, 2025. Licensed under [MIT](LICENSE) license.
