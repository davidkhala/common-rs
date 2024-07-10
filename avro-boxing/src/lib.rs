use std::convert::TryFrom;

use apache_avro::Decimal;
use bigdecimal::BigDecimal;
use bigdecimal::num_bigint::{BigInt};
use bigdecimal::num_traits::FromBytes;

pub fn to_bigdecimal(avro_d: Decimal) -> BigDecimal {
    let bytes = <Vec<u8>>::try_from(avro_d).unwrap();

    return BigDecimal::from(BigInt::from_be_bytes(&bytes));
}

#[cfg(test)]
mod tests {
    use bigdecimal::num_traits::{ToBytes};
    use bigdecimal::num_bigint::{ToBigInt};
    use super::*;

    fn bigint_100() -> BigInt {
        return 100.to_bigint().unwrap();
    }
    #[test]
    fn test_decimal_from_int() {
        let dec = BigDecimal::from(1);
        println!("{}", dec);
    }

    #[test]
    fn test_decimal_from_bigint() {
        let dec = BigDecimal::from(bigint_100());
        println!("{}", dec);
    }

    #[test]
    fn test_decimal_from_bytes_from_ref_decimal() {
        let input = bigint_100().to_be_bytes();
        let d = Decimal::from(input.clone());

        let output = <Vec<u8>>::try_from(&d).unwrap();
        assert_eq!(output, input);
    }

    #[test]
    fn test_decimal_from_bytes_from_owned_decimal() {
        let input = bigint_100().to_be_bytes();
        let d = Decimal::from(input.clone());

        let output = <Vec<u8>>::try_from(d).unwrap();
        assert_eq!(output, input.clone());
    }

    #[test]
    fn test_avro2big() {
        let avro_d = Decimal::from(bigint_100().to_be_bytes());
        let bytes = <Vec<u8>>::try_from(avro_d).unwrap();


        let big_decimal = BigDecimal::from(BigInt::from_be_bytes(&bytes));
        assert_eq!(big_decimal.to_string(), "100");
    }
}
