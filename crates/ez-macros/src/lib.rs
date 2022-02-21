#![warn(unused_crate_dependencies)]

pub use ez_proc_macro as proc;

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
macro_rules! doop {
    {
        $(let $named_binding:ident =  $($renamed_bindings:ident $(+)?)* $([$($named_replacements:tt),*])?;)*

        $(
            $(for $loop_bindings:tt in $($named_loop:ident $(+)?)* $([$($replacements:tt),*])?)+
            {
                $($rest:tt)*
            }
        )+
    } => {
        $crate::proc::doop!{
            [
                $([
                    [ $named_binding ]
                    [ $($renamed_bindings)* ]
                    [ $($( $named_replacements )*)? ]
                ])*
            ]
            $([
                [
                    $([
                        [ $loop_bindings ]
                        [ $( $named_loop )* ]
                        [ $($( $replacements )*)? ]
                    ])*
                ]
                [$($rest)*]
            ])+
        }
    }
}
