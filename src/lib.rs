use bigdecimal::{BigDecimal, FromPrimitive};

pub enum Unit {
    Wei(u128),
    Gwei(u128),
    Ether(u128),
}

impl Unit {
    pub fn one_eth_in_wei() -> BigDecimal {
        BigDecimal::from_f64(1e18f64).unwrap_or_default()
    }
    fn one_gwei_in_wei() -> BigDecimal {
        BigDecimal::from_f64(1e9f64).unwrap_or_default()
    }
    pub fn one_wei() -> BigDecimal {
        BigDecimal::from_u8(1u8).unwrap_or_default()
    }

    pub fn to_eth_str(self) -> Option<String> {
        match self {
            Self::Wei(n) => {
                if let Some(n) = BigDecimal::from_u128(n) {
                    return Some((n / Unit::one_eth_in_wei()).normalized().to_string())
                }
                None
            },
            Self::Gwei(n) => {
                if let Some(n) = BigDecimal::from_u128(n) {
                    return Some(((n * Unit::one_gwei_in_wei()) / Unit::one_eth_in_wei()).normalized().to_string())
                }
                None
            },
            Self::Ether(n) => Some(n.to_string()),
        }
    }

    pub fn to_gwei_str(self) -> Option<String> {
        match self {
            Self::Wei(n) => {
                if let Some(n) = BigDecimal::from_u128(n) {
                    return Some((n / Unit::one_gwei_in_wei()).normalized().to_string())
                }
                None
            },
            Self::Ether(n) => {
                if let Some(n) = BigDecimal::from_u128(n) {
                    return Some(((n * Unit::one_eth_in_wei()) / Unit::one_gwei_in_wei()).normalized().to_string())
                }
                None
            },
            Self::Gwei(n) => Some(n.to_string()),
        }
    }

    pub fn to_wei_str(self) -> Option<String> {
        match self {
           Self::Gwei(n) => {
                if let Some(n) = BigDecimal::from_u128(n) {
                    return Some((n * Unit::one_gwei_in_wei()).normalized().to_string())
                }
                None
            },
            Self::Ether(n) => {
                if let Some(n) = BigDecimal::from_u128(n) {
                    return Some((n * Unit::one_eth_in_wei()).normalized().to_string())
                }
                None
            },
            Self::Wei(n) => Some(n.to_string()),
        }
    }
}