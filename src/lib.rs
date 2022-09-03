use std::str::FromStr;
use bigdecimal::{BigDecimal, FromPrimitive};

pub enum Unit<'a>  {
    Wei(&'a str),
    Gwei(&'a str),
    Ether(&'a str),
}

impl<'a> Unit<'a> {
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
                if let Ok(n) = BigDecimal::from_str(n) {
                    return Some((n / Unit::one_eth_in_wei()).normalized().to_string())
                }
                None
            },
            Self::Gwei(n) => {
                if let Ok(n) = BigDecimal::from_str(n) {
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
                if let Ok(n) = BigDecimal::from_str(n) {
                    return Some((n / Unit::one_gwei_in_wei()).normalized().to_string())
                }
                None
            },
            Self::Ether(n) => {
                if let Ok(n) = BigDecimal::from_str(n) {
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
                if let Ok(n) = BigDecimal::from_str(n) {
                    return Some((n * Unit::one_gwei_in_wei()).normalized().to_string())
                }
                None
            },
            Self::Ether(n) => {
                if let Ok(n) = BigDecimal::from_str(n) {
                    return Some((n * Unit::one_eth_in_wei()).normalized().to_string())
                }
                None
            },
            Self::Wei(n) => Some(n.to_string()),
        }
    }
}