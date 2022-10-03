// You can use the 'use' keyword to bring module paths from modules from anywhere
// and especially from the Rust standard library into your scope.
// Bring SystemTime and UNIX_EPOCH

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    #[test]
    fn test_modules() {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }
}
