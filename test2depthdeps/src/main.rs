// prove direct access to 2 depth dependency is possible.
fn main() {}


#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use bigdecimal::num_bigint::{BigInt, ToBigInt};

    fn bigint_100() -> BigInt {
        return 100.to_bigint().unwrap();
    }
    #[test]
    fn test_decimal_from_int() {
        let dec = BigDecimal::from(1);
        println!("{}", dec);
    }
}

