# web3-unit-converter

Convert u128 to eth unit

## Getting Started

```rust
use web3_unit_converter::Unit;

fn main() {
    // Convert 1 Wei to ETH
    let one_wei_in_eth = Unit::Wei(1u128).to_eth_str().unwrap();
    println!("One Wei in ETH is: {}", one_wei_in_eth);
    // One Wei in ETH is: 0.000000000000000001


    // Convert 1 Gwei to ETH
    let one_gwei_in_eth = Unit::Gwei(1u128).to_eth_str().unwrap();
    println!("One Gwei in ETH is: {}", one_gwei_in_eth);
    // One Gwei in ETH is: 0.000000001


    // Convert 1 Eth to Gwei
    let one_eth_in_gwei = Unit::Ether(1u128).to_gwei_str().unwrap();
    println!("One ETH in Gwei is: {}", one_eth_in_gwei);
    // One ETH in Gwei is: 1000000000

    
    // Convert 1 Gwei to Wei
    let one_gwei_in_wei = Unit::Gwei(1u128).to_wei_str().unwrap();
    println!("One Gwei in Wei is: {}", one_gwei_in_wei);
    // One Gwei in Wei is: 1000000000
}
```

## Running the example

```
$ git clone https://github.com/p0p3yee/rust-web3-unit-converter
$ cd rust-web3-unit-converter
$ cargo run --example main

One Wei in ETH is: 0.000000000000000001
One Gwei in ETH is: 0.000000001
One ETH in Gwei is: 1000000000
One Gwei in Wei is: 1000000000
```