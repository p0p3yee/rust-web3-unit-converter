use web3_unit_converter::Unit;

fn main() {
    // Convert 1 Wei to ETH
    let one_wei_in_eth = Unit::Wei(&1u128.to_string()).to_eth_str().unwrap();
    println!("One Wei in ETH is: {}", one_wei_in_eth);
    // One Wei in ETH is: 0.000000000000000001


    // Convert 1 Gwei to ETH
    let one_gwei_in_eth = Unit::Gwei(&1u128.to_string()).to_eth_str().unwrap();
    println!("One Gwei in ETH is: {}", one_gwei_in_eth);
    // One Gwei in ETH is: 0.000000001


    // Convert 1 Eth to Gwei
    let one_eth_in_gwei = Unit::Ether(&1u128.to_string()).to_gwei_str().unwrap();
    println!("One ETH in Gwei is: {}", one_eth_in_gwei);
    // One ETH in Gwei is: 1000000000


    // Convert 1 Gwei to Wei
    let one_gwei_in_wei = Unit::Gwei(&1u128.to_string()).to_wei_str().unwrap();
    println!("One Gwei in Wei is: {}", one_gwei_in_wei);
    // One Gwei in Wei is: 1000000000


    // 15 decimals
    println!("1.111111111111112 ETH(float, 15 decimals) in Wei is: {}", Unit::Ether(&(1.111111111111112f64).to_string()).to_wei_str().unwrap());
    // 1.111111111111112 ETH(float, 15 decimals) in Wei is: 1111111111111112000


    // 16 decimals
    println!("1.1111111111111112 ETH(float, 16 decimals) in Wei is: {}", Unit::Ether(&(1.1111111111111112f64).to_string()).to_wei_str().unwrap());
    // 1.1111111111111112 ETH(float, 16 decimals) in Wei is: 1111111111111111200
    

    // When working with decimal more than 16, remember Not to parse the value from float to string
    // f64 only support up to 16 decimals.
    // 17 decimals
    println!("1.11111111111111112 ETH(float, 17 decimals) in Wei is: {}", Unit::Ether(&(1.11111111111111112f64).to_string()).to_wei_str().unwrap());
    // 1.11111111111111112 ETH(float, 17 decimals) in Wei is: 1111111111111111200
    // Correct result should be 1111111111111111120


    // 18 decimals
    println!("1.111111111111111112 ETH(float, 18 decimals) in Wei is: {}", Unit::Ether(&(1.111111111111111112f64).to_string()).to_wei_str().unwrap());
    // 1.111111111111111112 ETH(float, 18 decimals) in Wei is: 1111111111111111200
    // Correct result should be 1111111111111111112

    
    println!("1.111111111111111112 ETH(string) in Wei is: {}", Unit::Ether("1.111111111111111112").to_wei_str().unwrap());
    // 1.111111111111111112 ETH(string) in Wei is: 1111111111111111112
}
