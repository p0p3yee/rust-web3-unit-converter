#[cfg(test)]
mod tests {
    use web3_unit_converter::Unit;

    #[test]
    fn from_wei_to_eth_1() {
        let result = Unit::Wei(&1_000_000_000_000_000_000u128.to_string()).to_eth_str().unwrap();
        assert_eq!(result, "1");
    }

    #[test]
    fn from_wei_to_eth_2() {
        let result = Unit::Wei(&1_111_111_111_111_111_119u128.to_string()).to_eth_str().unwrap();
        assert_eq!(result, "1.111111111111111119");
    }

    #[test]
    fn from_wei_to_gwei_1() {
        let result = Unit::Wei(&1_111_111_111_111_111_119u128.to_string()).to_gwei_str().unwrap();
        assert_eq!(result, "1111111111.111111119");
    }

    #[test]
    fn from_wei_to_wei_1() {
        let result = Unit::Wei(&1_111_111_111_111_111_119u128.to_string()).to_wei_str().unwrap();
        assert_eq!(result, "1111111111111111119");
    }

    // From Gwei
    #[test]
    fn from_gwei_to_eth_1() {
        let result = Unit::Gwei(&1_000_000_000u128.to_string()).to_eth_str().unwrap();
        assert_eq!(result, "1");
    }

    #[test]
    fn from_gwei_to_eth_2() {
        let result = Unit::Gwei(&2_000_000_000_000u128.to_string()).to_eth_str().unwrap();
        assert_eq!(result, "2000");
    }

    #[test]
    fn from_gwei_to_gwei_1() {
        let result = Unit::Gwei(&1_000_000_000u128.to_string()).to_gwei_str().unwrap();
        assert_eq!(result, "1000000000");
    }

    #[test]
    fn from_gwei_to_wei_1() {
        let result = Unit::Gwei(&1_000_000_000u128.to_string()).to_wei_str().unwrap();
        assert_eq!(result, "1000000000000000000");
    }

    // From ETH
    #[test]
    fn from_eth_to_wei_1() {
        let result = Unit::Ether(&2_000_000u128.to_string()).to_wei_str().unwrap();
        assert_eq!(result, "2000000000000000000000000");
    }

    #[test]
    fn from_bigger_eth_to_wei_1() {
        //2,000,000,000,000
        let result = Unit::Ether(&2_000_000_000_000u128.to_string()).to_wei_str().unwrap();
        assert_eq!(result, "2000000000000000000000000000000");
    }

    #[test]
    fn from_bigger_eth_to_wei_2() {
        //9,000,000,000,000,000
        let result = Unit::Ether(&9_000_000_000_000_000_000u128.to_string()).to_wei_str().unwrap();
        assert_eq!(result, "9000000000000000000000000000000000000");
    }

    #[test]
    fn from_eth_to_gwei_1() {
        //9,000,000,000,000,000
        let result = Unit::Ether(&9_000_000_000_000_000_000u128.to_string()).to_gwei_str().unwrap();
        assert_eq!(result, "9000000000000000000000000000");
    }

    #[test]
    fn from_eth_to_eth_1() {
        //9,000,000,000,000,000
        let result = Unit::Ether(&9_000_000_000_000_000_000u128.to_string()).to_eth_str().unwrap();
        assert_eq!(result, "9000000000000000000");
    }

    // From 0 test
    #[test]
    fn from_zero_eth_to_eth() {
        let result = Unit::Ether(&0u128.to_string()).to_eth_str().unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn from_zero_eth_to_wei() {
        let result = Unit::Ether(&0u128.to_string()).to_wei_str().unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn from_zero_eth_to_gwei() {
        let result = Unit::Ether(&0u128.to_string()).to_gwei_str().unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn from_zero_gwei_to_gwei() {
        let result = Unit::Gwei(&0u128.to_string()).to_gwei_str().unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn from_zero_gwei_to_wei() {
        let result = Unit::Gwei(&0u128.to_string()).to_wei_str().unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn from_zero_gwei_to_eth() {
        let result = Unit::Gwei(&0u128.to_string()).to_eth_str().unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn from_zero_wei_to_gwei() {
        let result = Unit::Wei(&0u128.to_string()).to_gwei_str().unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn from_zero_wei_to_wei() {
        let result = Unit::Wei(&0u128.to_string()).to_wei_str().unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn from_zero_wei_to_eth() {
        let result = Unit::Wei(&0u128.to_string()).to_eth_str().unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn from_one_wei_to_eth() {
        let result = Unit::Wei(&1u128.to_string()).to_eth_str().unwrap();
        assert_eq!(result, "0.000000000000000001");
    }

    #[test]
    fn from_one_gwei_to_eth() {
        let result = Unit::Gwei(&1u128.to_string()).to_eth_str().unwrap();
        assert_eq!(result, "0.000000001");
    }

    #[test]
    fn from_one_eth_to_eth() {
        let result = Unit::Ether(&1u128.to_string()).to_eth_str().unwrap();
        assert_eq!(result, "1");
    }

}