
/// Convert to Int infallibly and exactly.
pub trait ToInt {
    fn to_int(&self) -> crate::Int;
}

/// Try to convert to Int exactly.
pub trait TryToInt {
    fn try_to_int(&self) -> Result<crate::Int, ez::Error>;
}

/// Convert to Int infallibly but with possible rounding.
/// (This is not saturating; out-of-bounds values will fail to convert.)
pub trait ToIntApproximate: TryToIntApproximate {
    fn to_int_approximate(&self) -> crate::Int;
}

/// Try to convert to Int with possible rounding.
pub trait TryToIntApproximate {
    fn try_to_int_approximate(&self) -> Result<crate::Int, ez::Error>;
}

/// Convert from Int infallibly and exactly.
pub trait FromInt: Sized {
    fn from_int(i: &crate::Int) -> Self;
}

/// Try to convert from Int exactly.
pub trait TryFromInt: Sized {
    fn try_from_int(i: &crate::Int) -> Result<Self, ez::Error>;
}

/// Convert from Int infallibly but with possible rounding.
/// (This is not saturating; out-of-bounds values will fail to convert.)
pub trait FromIntApproximate: Sized {
    fn from_int_approximate(i: &crate::Int) -> Self;
}

/// Try to convert from Int with possible rounding.
pub trait TryFromIntApproximate: Sized {
    fn try_from_int_approximate(i: &crate::Int) -> Result<Self, ez::Error>;
}

impl<ToIntImpl: ToInt> TryToInt for ToIntImpl {
    fn try_to_int(&self) -> Result<crate::Int, ez::Error> {
        Ok(self.to_int())
    }
}

impl<ToIntImpl: ToInt> ToIntApproximate for ToIntImpl {
    fn to_int_approximate(&self) -> crate::Int {
        self.to_int()
    }
}

impl<ToIntApproximateImpl: ToIntApproximate> TryToIntApproximate for ToIntApproximateImpl {
    fn try_to_int_approximate(&self) -> Result<crate::Int, ez::Error> {
        Ok(self.to_int_approximate())
    }
}

impl From<crate::Int> for i128 {
    fn from(val: crate::Int) -> Self {
        val.0
    }
}


impl<T: ToInt> std::convert::From<T> for crate::Int {
    fn from(t: T) -> crate::Int {
        t.to_int()
    }
}

