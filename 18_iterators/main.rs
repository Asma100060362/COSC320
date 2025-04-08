#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    DivideByZero,
    IntegerOverflow,
    NotDivisible,
}

fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a == i64::MIN && b == -1 {
        Err(DivisionError::IntegerOverflow)
    } else if a % b != 0 {
        Err(DivisionError::NotDivisible)
    } else {
        Ok(a / b)
    }
}

fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    numbers.iter().map(|&n| divide(n, 27)).collect()
}

fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    numbers.iter().map(|&n| divide(n, 27)).collect()
}

fn main() {
    // You can optionally experiment here
}
