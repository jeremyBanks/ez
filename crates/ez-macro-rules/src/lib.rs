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
}

#[macro_export]
macro_rules! repeat {
    {
        $(for $id:ident in [$($replacement:ident),*])+
        {
            $($rest:tt)*
        }
    } => {
        ::ez::__::repeat_impl!{
            [
                $([
                    $id
                    {$( $replacement )*}
                ])*
            ]
            [$($rest)*]
        }
    }
}
