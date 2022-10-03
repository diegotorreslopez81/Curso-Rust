// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.

mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}


#[cfg(test)]
mod tests {
    use crate::delicious_snacks;

    #[test]
    fn test_modules() {
        println!(
            "favorite snacks: {} and {}",
            delicious_snacks::fruit,
            delicious_snacks::veggie
        );
    }
}
