use eyre::{eyre, WrapErr};

#[derive(
    Clone,
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Default,
    derive_more::Display,
    derive_more::DebugCustom,
    derive_more::From,
    derive_more::Into,
    derive_more::Constructor,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
    derive_more::MulAssign,
    derive_more::Div,
    derive_more::DivAssign,
    derive_more::Rem,
    derive_more::RemAssign,
    derive_more::Shr,
    derive_more::ShrAssign,
    derive_more::Shl,
    derive_more::ShlAssign,
    derive_more::Neg,
)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Int {
    inner: i128,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct IntExponent(Int);

impl std::ops::Deref for Int {
    type Target = IntExponent;
    fn deref(&self) -> &IntExponent {
        unsafe { std::mem::transmute(self) }
    }
}

impl std::ops::Mul<IntExponent> for Int {
    type Output = Int;
    fn mul(self, other: IntExponent) -> Int {
        Int::new(
            self.inner.pow(
                other
                    .0
                    .inner
                    .try_into()
                    .wrap_err("exponent out-of-bounds")
                    .unwrap(),
            ),
        )
    }
}

impl std::ops::Mul<Fallible<Int>> for Int {
    type Output = Int;
    fn mul(self, other: Fallible<Int>) -> Int {
        self.mul(other.wrap_err("attempting to multiply").unwrap())
    }
}

impl std::ops::Mul<Int> for Int {
    type Output = Int;
    fn mul(self, other: Int) -> Int {
        Int::new(self.inner.mul(other.inner))
    }
}

pub type Fallible<T> = Result<T, eyre::Report>;
// Should we impl Add with Fallible<Int>, auto-unwrapping? Yes.

pub trait CoerceToInt {
    fn try_to_int(&self) -> Fallible<Int>;
}

impl CoerceToInt for Int {
    fn try_to_int(&self) -> Fallible<Int> {
        Ok(*self)
    }
}

impl CoerceToInt for i128 {
    fn try_to_int(&self) -> Fallible<Int> {
        Ok((*self).into())
    }
}

impl CoerceToInt for f64 {
    fn try_to_int(&self) -> Fallible<Int> {
        let original = (*self).trunc();
        let cast = original as i128;
        let restored = cast as f64;
        if original == restored {
            Ok(cast.into())
        } else {
            Err(eyre::eyre!(
                "f64 value ({:e}) was out-of-bounds for i128",
                original
            ))
        }
    }
}

impl CoerceToInt for &str {
    fn try_to_int(&self) -> Fallible<Int> {
        Ok(self.parse::<i128>()?.into())
    }
}

impl CoerceToInt for String {
    fn try_to_int(&self) -> Fallible<Int> {
        Ok(self.parse::<i128>()?.into())
    }
}

impl CoerceToInt for bool {
    fn try_to_int(&self) -> Fallible<Int> {
        Ok((if *self { 1 } else { 0 }).into())
    }
}

pub fn int(value: impl CoerceToInt) -> Int {
    value.try_to_int().unwrap()
}

fn main() {
    global_init();

    println!("{:?}", int(2) * *int(8));
    println!("{:?}", int(2) * 1e100.try_to_int());
}

fn global_init() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        if let Err(err) = dotenv::dotenv() {
            tracing::error!("Failed to initialize dotenv (.env file loading): {}", err);
        }
        if let Err(err) = color_eyre::install() {
            tracing::error!("Failed to initialize eyre (error formatting): {}", err);
        }
        if let Err(err) = tracing_subscriber::util::SubscriberInitExt::try_init(
            tracing_subscriber::Layer::with_subscriber(
                tracing_error::ErrorLayer::default(),
                tracing_subscriber::fmt()
                    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                    .with_target(false)
                    .with_span_events(
                        tracing_subscriber::fmt::format::FmtSpan::NEW
                            | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
                    )
                    .finish(),
            ),
        ) {
            tracing::error!("Failed to initialize tracing (logging): {}", err);
        }
    })
}
