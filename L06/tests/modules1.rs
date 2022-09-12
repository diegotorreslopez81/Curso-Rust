
// I AM NOT DONE

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_modules() {
        sausage_factory::make_sausage();
    }
}
