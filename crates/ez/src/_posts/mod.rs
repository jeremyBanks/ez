use paste::paste;

macro_rules! posts {
    {
        $(
            $(
                $year:literal-$month:literal-$day:literal $($name:ident)*
            ),+ $(,)?
        )+
    } => {
        $(
            $(
                paste!{
                    pub mod [< _ $year _ $month _ $day $(_ $name)* >] {
                        #![doc = include_str!(concat!(
                            stringify!($year),
                            "-",
                            stringify!($month),
                            "-",
                            stringify!($day),
                            $(
                            "-",
                            stringify!($name),
                            )*
                            ".md"
                        ))]
                    }
                }
            )+
        )+
    }
}

posts![2022 - 03 - 18,];
