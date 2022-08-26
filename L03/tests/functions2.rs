// Don't mind this for now :)
// I AM NOT DONE




#[cfg(test)]
mod tests {

    fn call_me(num: u32) {
        for i in 0..num {
            println!("Ring! Call number {}", i + 1);
        }
    }

    #[test]
    fn call_function() {
        call_me(3);
    }

}