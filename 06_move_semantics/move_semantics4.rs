fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // Fixed by reordering the lines (no line added, changed, or removed)
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        
        let y = &mut x;
        y.push(42);           // Use `y` first, then release it

        let z = &mut x;       // Now we can borrow `x` again
        z.push(13);           // Use `z` after `y` is done
        
        assert_eq!(x, [42, 13]);
    }
}
