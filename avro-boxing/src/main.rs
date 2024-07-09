use std::convert::TryFrom;

use apache_avro::Decimal;
use bigdecimal::BigDecimal;
use bigdecimal::num_bigint::{BigInt, ToBigInt};

fn main() {
    let dec = BigDecimal::from(1);

    let bd = BigDecimal::from(100.to_bigint().unwrap());
    println!("1 ({}), 100 ({})", dec, bd);


    let decimal = Decimal::from(100_i8.to_be_bytes());
    let maybe_bytes = <Vec<u8>>::try_from(&decimal);
    println!("{}", BigInt::from_signed_bytes_be(&maybe_bytes.unwrap()));
}




#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use bigdecimal::num_traits::ToBytes;

    use super::*;

    #[test]
    fn test_decimal_from_bytes_from_ref_decimal() {
        let input = 100_i8.to_be_bytes();
        let d = Decimal::from(input);

        let output = <Vec<u8>>::try_from(&d).unwrap();
        assert_eq!(output, input);

    }

    #[test]
    fn test_decimal_from_bytes_from_owned_decimal() {
        let input = 100_i8.to_be_bytes();
        let d = Decimal::from(input);

        let output = <Vec<u8>>::try_from(d).unwrap();
        assert_eq!(output, input);
    }
}
