// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    (1..=num).fold(1, |acc, n| acc * n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

























// In an imperative language you might write a for loop to iterate through
// multiply the values into a mutable variable. Or you might write code more
// functionally with recursion and a match clause. But you can also use ranges
// and iterators to solve this in rust.
