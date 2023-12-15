use ethers::types::U256;
use ethers::utils::format_units;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Mul};

pub type Decimals = u8;

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Ord, Default)]
pub struct Amount {
    pub base_units: U256,
    pub decimals: Decimals,
}

impl Amount {
    pub fn from_base_units(base_units: U256, decimals: Decimals) -> Self {
        Amount {
            base_units,
            decimals,
        }
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.decimals, rhs.decimals);
        Amount {
            base_units: self.base_units + rhs.base_units,
            decimals: self.decimals,
        }
    }
}

impl Mul for Amount {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.decimals, rhs.decimals);
        Amount {
            base_units: self.base_units * rhs.base_units,
            decimals: self.decimals * 2,
        }
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match format_units(self.base_units, self.decimals as u32) {
            Ok(result) => write!(f, "{}", result),
            Err(error) => {
                writeln!(f, "Error formatting units {}", error)?;
                Err(fmt::Error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_add() {
        let amount1 = Amount {
            base_units: U256::from_dec_str("1000").unwrap(),
            decimals: 2,
        };

        let amount2 = Amount {
            base_units: U256::from_dec_str("2000").unwrap(),
            decimals: 2,
        };

        let expected_sum = Amount {
            base_units: U256::from_dec_str("3000").unwrap(),
            decimals: 2,
        };

        let result = amount1 + amount2;
        assert_eq!(result, expected_sum);
    }

    #[test]
    #[should_panic]
    fn test_amount_addition_different_decimals() {
        let amount1 = Amount {
            base_units: U256::from_dec_str("1000").unwrap(),
            decimals: 2,
        };

        let amount2 = Amount {
            base_units: U256::from_dec_str("2000").unwrap(),
            decimals: 3,
        };

        let _result = amount1 + amount2;
    }

    #[test]
    fn test_amount_mul() {
        let amount1 = Amount {
            base_units: U256::from_dec_str("1000").unwrap(),
            decimals: 2,
        };

        let amount2 = Amount {
            base_units: U256::from_dec_str("2000").unwrap(),
            decimals: 2,
        };

        let expected_product = Amount {
            base_units: U256::from_dec_str("2000000").unwrap(),
            decimals: 4,
        };

        let result = amount1 * amount2;
        assert_eq!(result, expected_product);
    }

    #[test]
    #[should_panic]
    fn test_amount_multiplication_different_decimals() {
        let amount1 = Amount {
            base_units: U256::from_dec_str("1000").unwrap(),
            decimals: 2,
        };

        let amount2 = Amount {
            base_units: U256::from_dec_str("2000").unwrap(),
            decimals: 3,
        };

        let _result = amount1 * amount2;
    }

    #[test]
    fn test_amount_display() {
        let amount = Amount {
            base_units: U256::from_dec_str("12345678901234567890").unwrap(),
            decimals: 8,
        };
        assert_eq!(amount.to_string(), "123456789012.34567890");
    }
}
