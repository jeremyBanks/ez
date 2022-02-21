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
        $( let $let_binding_name:ident = $( *$let_binding_refs:ident $(+)? )*
                                         $( [ $( $let_binding_replacements:tt ),* ] )?
                                         $( + )?
                                         $( braced [ $( { $( $let_binding_braced_replacements:tt )* } ),* ] )?
                                         $( ; )? )*

        $(
            $( for $loop_binding_name:tt in $( *$loop_binding_refs:ident $(+)? )*
                                            $( [ $( $loop_binding_replacements:tt ),* ] )?
                                            $( + )?
                                            $( braced [ $( { $( $loop_binding_braced_replacements:tt )* } ),* ] )?
                                            $( : )? )*
            {
                $( $body:tt )*
            }
        )+
    } => {
        $crate::proc::doop!{
            (doop_args (let_bindings $( (let_binding (let_binding_name $let_binding_name)
                                                     (let_binding_refs $( $let_binding_refs )*)
                                                     (let_binding_replacements $( $( $let_binding_replacements )* )? )
                                                     (let_binding_braced_replacements $( $( $let_binding_braced_replacements )* )? )) )* ))
                       (loops $( (loop (loop_bindings $( (loop_binding (loop_binding_name $loop_binding_name)
                                                                       (loop_binding_refs $( $loop_binding_refs )* )
                                                                       (loop_binding_replacements $( $( $loop_binding_replacements )* )? )
                                                                       (loop_binding_braced_replacements $( $( $loop_binding_braced_replacements )* )? )) )* )
                                       (body $($body)* ) ) )* )
        }
    }
}
