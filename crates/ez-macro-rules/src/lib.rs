#![warn(unused_crate_dependencies)]

#[macro_export]
macro_rules! throw {
    ($msg:literal $(,)?) => { {
        ::ez::__::Err(::ez::Error::msg($msg))?;
        unreachable!()
    } };

    ($msg:literal $(, $rest:tt)* $(,)?) => { {
        ::ez::__::Err(::ez::Error::msg(format!($msg $(, $rest)*)))?;
        unreachable!()
    } };

    ($error:expr $(,)?) => { {
        ::ez::__::Err($error)?;
        unreachable!()
    } };

    ($(,)?) => { {
        ::ez::__::Err(::ez::__::core::default::Default::default())?;
        unreachable!()
    } };
}

#[macro_export]
macro_rules! publish {
    {
        pub use $path:path $(as $name:ident)?;
        $(prose from $doc:literal;)*
        $(example $example:ident;)*
        $(failing example $failing:ident;)*
    } => {
        $(
        ///
        #[doc = include_str!(concat!("./", $doc))]
        ///
        )*
        $(
        ///
        #[doc = concat!("## Example `", stringify!($example), "`")]
        ///
        /// ```rust
        /// # use ez::__::*;
        #[doc = include_str!(concat!("../examples/", stringify!($example), ".rs"))]
        /// ```
        )*
        $(
        ///
        #[doc = concat!("## Example `", stringify!($failing), "`")]
        ///
        /// ```should_panic
        /// # use ez::__::*;
        #[doc = include_str!(concat!("../examples/", stringify!($failing), ".rs"))]
        /// ```
        )*
        ///
        #[doc(inline)]
        pub use $path $(as $name)?;
    }
}
