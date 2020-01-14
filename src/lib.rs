//! This crate provides a container for a value, `DirtyStatic`, which
//! allows mutation in debug mode (via `UnsafeCell`), but not in
//! release mode.
//!
//! This allows you to tweak data while testing an application,
//! without having that data be mutable when the application is
//! released.
//!
//! There are also two features available:
//!
//! 1. `force-dynamic` which allows replacing the value of a
//!     `DirtyStatic` even in release mode.
//! 2. `force-static` which disallows replacing the value of a
//!     `DirtyStatic` even in debug mode.

#[cfg(all(feature = "force-static", feature = "force-dynamic"))]
compile_error!("dirty_static: Cannot enable both the force-static and force-dynamic features.");

pub use internal::DirtyStatic;

#[cfg(any(
    feature = "force-dynamic",
    all(not(feature = "force-static"), debug_assertions)
))]
mod internal {
    use std::cell::UnsafeCell;
    use std::ops::Deref;

    /// A container for a value which allows interior mutation
    /// only in debug mode. (Or when enabled via `force-dynamic`
    /// feature.)
    pub struct DirtyStatic<T>(UnsafeCell<T>);
    unsafe impl<T> Sync for DirtyStatic<T> where T: Sync {}

    impl<T> Deref for DirtyStatic<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            let ptr = self.0.get();
            unsafe { &*ptr }
        }
    }

    impl<T> DirtyStatic<T> {
        /// Create a new DirtyStatic with the given interior value.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use dirty_static::DirtyStatic;
        ///
        /// let c = DirtyStatic::new(100);
        /// assert_eq!(*c, 100);
        /// ```
        pub const fn new(t: T) -> Self {
            DirtyStatic(UnsafeCell::new(t))
        }

        /// Replace the interior value of the DirtyStatic. Note that
        /// this will do nothing unless running in debug mode, or
        /// enabling the `force-dynamic` feature.
        ///
        /// # Safety
        ///
        /// When calling `replace`, any references to data inside the DirtyStatic
        /// are invalidated. Accessing this data is undefined behaviour.
        ///
        /// For this reason, it's a good idea to only ever hold references to this
        /// data between calls to replace. For example, if you replace the data
        /// every frame, make sure you do not hold a reference across two frames.
        ///
        /// If you do need to reference data across this boundary, do so indirectly.
        /// For example, store a HashMap in the DirtyStatic, and hold onto a key
        /// instead of a reference.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// // In debug mode
        /// use dirty_static::DirtyStatic;
        ///
        /// let c = DirtyStatic::new(100);
        /// unsafe {
        ///     c.replace(200);
        /// }
        ///
        /// assert_eq!(*c, 200);
        /// ```
        ///
        /// ```rust,no_run
        /// // In release mode
        /// use dirty_static::DirtyStatic;
        ///
        /// let c = DirtyStatic::new(100);
        /// unsafe {
        ///     c.replace(200);
        /// }
        ///
        /// assert_eq!(*c, 100);
        /// ```
        pub unsafe fn replace(&self, t: T) {
            let ptr = self.0.get();
            *ptr = t
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn create_value() {
            let text = "Hello".to_string();
            let c = DirtyStatic::new(text);

            assert_eq!(&*c, "Hello");
        }

        #[test]
        fn refresh_value() {
            let text = "Hello".to_string();
            let c = DirtyStatic::new(text);

            unsafe { c.replace("Replacement value".to_string()) };
            assert_eq!(&*c, "Replacement value");
        }
    }
}

#[cfg(any(
    feature = "force-static",
    all(not(feature = "force-dynamic"), not(debug_assertions))
))]
mod internal {
    use std::ops::Deref;

    pub struct DirtyStatic<T>(T);

    impl<T> Deref for DirtyStatic<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DirtyStatic<T> {
        pub const fn new(t: T) -> Self {
            DirtyStatic(t)
        }

        pub unsafe fn replace(&self, _t: T) {
            eprintln!("WARNING: Can't replace in release mode!");
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn create_value() {
            let text = "Hello".to_string();
            let c = DirtyStatic::new(text);

            assert_eq!(&*c, "Hello");
        }

        #[test]
        fn refresh_value_does_nothing() {
            let text = "Hello".to_string();
            let c = DirtyStatic::new(text);

            unsafe { c.replace("Replacement value".to_string()) };
            assert_eq!(&*c, "Hello");
        }
    }
}

#[cfg(test)]
mod feature_tests {
    use super::DirtyStatic;

    fn _assert_static() {
        let c = DirtyStatic::new(10);
        unsafe { c.replace(20) };
        assert_eq!(*c, 10);
    }

    fn _assert_dynamic() {
        let c = DirtyStatic::new(10);
        unsafe { c.replace(20) };
        assert_eq!(*c, 20);
    }

    #[test]
    #[cfg(all(
        debug_assertions,
        not(any(feature = "force-static", feature = "force-dynamic"))
    ))]
    fn feature_test() {
        _assert_dynamic();
    }

    #[test]
    #[cfg(all(
        not(debug_assertions),
        not(any(feature = "force-static", feature = "force-dynamic"))
    ))]
    fn feature_test() {
        _assert_static();
    }

    #[test]
    #[cfg(all(debug_assertions, feature = "force-static"))]
    fn feature_test() {
        _assert_static();
    }

    #[test]
    #[cfg(all(debug_assertions, feature = "force-dynamic"))]
    fn feature_test() {
        _assert_dynamic();
    }

    #[test]
    #[cfg(all(not(debug_assertions), feature = "force-static"))]
    fn feature_test() {
        _assert_static();
    }

    #[test]
    #[cfg(all(not(debug_assertions), feature = "force-dynamic"))]
    fn feature_test() {
        _assert_dynamic();
    }
}
