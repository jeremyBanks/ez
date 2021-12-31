use crate::{delegations::*, *};

#[derive(Delegate)]
#[delegate(Display)]
#[delegate(Debug)]
pub struct Int(i128);
