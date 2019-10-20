#[cfg(all(feature = "force-static", feature = "force-dynamic"))]
compile_error!("dirty_const: Cannot enable both the force-static and force-dynamic features.");

pub use internal::DirtyConst;

#[cfg(any(
    feature = "force-dynamic",
    all(not(feature = "force-static"), debug_assertions)
))]
mod internal {
    use std::cell::UnsafeCell;
    use std::ops::Deref;

    pub struct DirtyConst<T>(UnsafeCell<T>);
    unsafe impl<T> Sync for DirtyConst<T> where T: Sync {}

    impl<T> Deref for DirtyConst<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            let ptr = self.0.get();
            unsafe { &*ptr }
        }
    }

    impl<T> DirtyConst<T> {
        pub const fn new(t: T) -> Self {
            DirtyConst(UnsafeCell::new(t))
        }

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
            let c = DirtyConst::new(text);

            assert_eq!(&*c, "Hello");
        }

        #[test]
        fn refresh_value() {
            let text = "Hello".to_string();
            let c = DirtyConst::new(text);

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

    pub struct DirtyConst<T>(T);

    impl<T> Deref for DirtyConst<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DirtyConst<T> {
        pub const fn new(t: T) -> Self {
            DirtyConst(t)
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
            let c = DirtyConst::new(text);

            assert_eq!(&*c, "Hello");
        }

        #[test]
        fn refresh_value_does_nothing() {
            let text = "Hello".to_string();
            let c = DirtyConst::new(text);

            unsafe { c.replace("Replacement value".to_string()) };
            assert_eq!(&*c, "Hello");
        }
    }
}

#[cfg(test)]
mod feature_tests {
    use super::DirtyConst;

    fn _assert_static() {
        let c = DirtyConst::new(10);
        unsafe { c.replace(20) };
        assert_eq!(*c, 10);
    }

    fn _assert_dynamic() {
        let c = DirtyConst::new(10);
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
