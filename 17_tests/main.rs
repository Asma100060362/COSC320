struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Don't change this function.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
        
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // Test if the rectangle has the correct size
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);  // Check width
        assert_eq!(rect.height, 20); // Check height
    }

    // Test if the program panics when we try to create a rectangle with negative width.
    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // Test if the program panics when we try to create a rectangle with negative height.
    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
