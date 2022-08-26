// Don't mind this for now :)
// I AM NOT DONE


#[cfg(test)]
mod tests {

fn call_me() {
    for i in 0..5 {
        println!("Ring! Call number {}", i + 1);
    }
}

    #[test]
    fn call_function() {
        call_me();
    }
}
