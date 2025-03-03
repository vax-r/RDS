pub use std::sync::Arc;
use std::{
    ops::{Deref, DerefMut},
};
use std::alloc::AllocError;
use std::mem::MaybeUninit;

/// A refcounted object that is known to have a refcount of 1.
///
/// It is mutable and can be converted to an [`Arc`] so that it can be shared.
///
/// # Invariants
///
/// `inner` always has a reference count of 1.
pub struct UniqueArc<T: ?Sized> {
    inner: Arc<T>,
}


impl<T> UniqueArc<T> {
    /// Tries to allocate a new [`UniqueArc`] instance.
    #[allow(dead_code)]
    pub fn new(value: T) -> Result<Self, AllocError> {
        Ok(Self {
            // INVARIANT: THe newly-created object has a ref-count of 1.
            inner: Arc::try_new(value)?,
        })
    }

    /// Tries to allocate a new [`UniqueArc`] instance whose contents are not initialised yet.
    #[allow(dead_code)]
    pub fn new_uninit() -> Result<UniqueArc<MaybeUninit<T>>, AllocError> {
        // INVARIANT: The refcount is initialised to a non-zero value.
        let inner_arc = Arc::<T>::try_new_uninit()?;

        Ok(UniqueArc::<MaybeUninit<T>> {
            inner: inner_arc,
        })
    }
}

impl<T> UniqueArc<MaybeUninit<T>> {
    /// Converts a `UniqueArc<MaybeUninit<T>>` into a `UniqueArc<T>` by writing a value into it.
    #[allow(dead_code)]
    pub fn write(mut self, value: T) -> UniqueArc<T> {
        Arc::get_mut(&mut self.inner).unwrap().write(value);
        // SAFETY: We just wrote the value to be initialised.
        unsafe { self.assume_init() }
    }

    /// Unsafely assume that `self` is initialized.
    ///
    /// # Safety
    ///
    /// The caller guarantees that the value behind this pointer has been initialized. It is
    /// *immediate* UB to call this when the value is not initialized.
    #[allow(dead_code)]
    pub unsafe fn assume_init(self) -> UniqueArc<T> {
        let inner = unsafe { self.inner.assume_init() };
        UniqueArc {
            inner: Arc::from_raw(Arc::into_raw(inner) as *const T),
        }
    }
}

impl<T: ?Sized> Deref for UniqueArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T: ?Sized> DerefMut for UniqueArc<T> {
    // SAFETY: By the `Arc` type invariant, there is necessarily a reference to the object, so
    // it is safe to dereference it. Additionally, we know there is only one reference when
    // it's inside a `UniqueArc`, so it is safe to get a mutable reference.
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::get_mut(&mut self.inner).unwrap()
    }
}

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

    #[test]
    fn test_unique_arc_new() {

        struct MockData {
            a: u32,
            b: u32,
        }

        let mut x = UniqueArc::new(MockData { a: 10, b: 20}).unwrap();
        assert_eq!(x.a, 10);
        assert_eq!(x.b, 20);

        x.a += 1;
        x.b += 1;
        assert_eq!(x.a, 11);
        assert_eq!(x.b, 21);
    }

    #[test]
    fn test_unique_arc_new_uninit() {
        #[allow(dead_code)]
        struct MockData {
            a: u32,
            b: u32,
        }

        let x = UniqueArc::<MockData>::new_uninit().unwrap();
        assert_eq!(Arc::strong_count(&x.inner), 1);
    }

    // FIXME : assume_init() doesn't triggered the UB even if the value isn't initialized
    // explicitly.
    #[test]
    fn test_unique_arc_write() {
        struct MockData {
            a: u32,
        }

        let x = unsafe { UniqueArc::<MockData>::new_uninit().unwrap() };
        let y = x.write(MockData{a: 10});

        assert_eq!(y.a, 10);
    }
}



