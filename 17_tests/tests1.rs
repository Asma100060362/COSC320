// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module

    #[test]
    fn you_can_assert() {
        // Test the function `is_even` with some values.
        assert!(is_even(2)); // 2 is even, so it should return true
        assert!(!is_even(3)); // 3 is odd, so it should return false
    }
}
