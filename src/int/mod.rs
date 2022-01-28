#![doc = include_str!("./README.md")]

#[derive(crate::internal::Int)]
pub struct Int(i128);

// #[try_or_unwrap]
// pub fn try_int(x: impl WeaklyInto<Int>) -> Int {
//     x.try_weakly_into()?
// }

// impl WeaklyFrom<u8> for Int {

//     fn weakly_from(x: u8) -> Int {
//         todo!()
//     }
// }

// impl WeaklyFrom<u16> for Int {
//     fn weakly_from(x: u16) -> Int {
//         todo!()
//     }
// }
