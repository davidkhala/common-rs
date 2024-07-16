fn main() {}
#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    #[test]
    fn test_decimal_from_int() {
        let dec = BigDecimal::from(1);
        println!("{}", dec);
    }
    #[test]
    fn get_sys_env() {
        use std::env;

        let mut handlers = vec![];
        let text ="".to_string();
        handlers.push(std::thread::spawn(move || {
            println!("{:?}", env::var_os("HOME").unwrap());
            println!("{}", text)
        }));

        handlers.push(std::thread::spawn(move || {
            println!("{:?}", env::var_os("HOME").unwrap());
            // TODO assert throw here
            println!("{}", text)
        }));
        for h in handlers {
            h.join().unwrap();
        }
    }
}
