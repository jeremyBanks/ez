#[macro_export]
macro_rules! throw {
    ($msg:literal $(,)?) => { {
        ::ez::__::core::result::Result::Err(::ez::Error::msg($msg))?;
        unreachable!()
    } };

    ($msg:literal $(, $rest:tt)* $(,)?) => { {
        ::ez::__::core::result::Result::Err(::ez::Error::msg(format!($msg $(, $rest)*)))?;
        unreachable!()
    } };

    ($error:expr $(,)?) => { {
        ::ez::__::core::result::Result::Err($error)?;
        unreachable!()
    } };

    ($(,)?) => { {
        ::ez::__::core::result::Result::Err(::ez::__::core::default::Default::default())?;
        unreachable!()
    } };
}
