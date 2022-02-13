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
        use $path:path $(as $name:ident)?;
        $(docs $doc:ident;)*
        $(example $example:ident;)*
        $(failing $failing:ident;)*
    } => {
        $(
        ///
        #[doc = include_str!(concat!("./", stringify!($doc), ".md"))]
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
