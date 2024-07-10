fn main() {

}
#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    #[test]
    fn test_decimal_from_int() {
        let dec = BigDecimal::from(1);
        println!("{}", dec);
    }
}
