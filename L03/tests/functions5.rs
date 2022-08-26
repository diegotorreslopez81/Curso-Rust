// I AM NOT DONE



#[cfg(test)]
mod tests {
    #[test]
    fn call_function() {
        let answer = square(3);
        println!("The square of 3 is {}", answer);
    }

    fn square(num: i32) -> i32 {
        let x = num * num;
        return x;
    }
}