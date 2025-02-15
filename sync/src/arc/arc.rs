pub use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc() {
        let data = Arc::new(5);
        let clone = data.clone();
        assert_eq!(*data, 5);
        assert_eq!(*clone, 5);
    }

    #[test]
    fn test_arc_refcount_value() {
        let data = Arc::new(5);
        let _garbage = Arc::clone(&data);

        // This assertion is deterministic because the "Arc"
        // or "Weak" haven't been shared between threads.
        assert_eq!(Arc::strong_count(&data), 2);
        assert_eq!(Arc::weak_count(&data), 0);
    }

    #[test]
    fn test_arc_refcount_value_with_downgrade() {
        let data = Arc::new(5);
        let _garbage = Arc::downgrade(&data);

        // This assertion is deterministic because the "Arc"
        // or "Weak" haven't been shared between threads.
        assert_eq!(Arc::strong_count(&data), 1);
        assert_eq!(Arc::weak_count(&data), 1);
    }
}



