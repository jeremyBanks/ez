use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

fn main() {
    let mut b = DebugBrush;
    b.stroke(0.5);

    eprintln!("Scaled up:");
    let mut b = b.scaled_up::<2>();
    b.stroke(0.5);
    b.rotate(0.5);

    eprintln!("Down more:");
    let mut b = b.scaled_down::<3>();
    b.stroke(0.5);
    b.rotate(0.5);

    eprintln!("Mirrored");
    let mut b = b.mirrored();
    b.stroke(0.5);
    b.rotate(0.5);

    eprintln!("Unwrapping original");
    let mut b = b.take() as &mut DebugBrush;
    b.stroke(0.5);
}

trait Brush {
    fn stroke(&mut self, scale: f64) -> &mut Self;

    fn rotate(&mut self, taus: f64) -> &mut Self;
}

#[derive(Debug)]
struct DebugBrush;

impl Brush for DebugBrush {
    fn stroke(&mut self, scale: f64) -> &mut Self {
        eprintln!("stroke({scale})");
        self
    }

    fn rotate(&mut self, taus: f64) -> &mut Self {
        eprintln!("rotate({taus})");
        self
    }
}

#[derive(Debug)]
struct Handle<Handler, Inner: Brush> {
    inner: Inner,
    handler: PhantomData<Handler>,
}

impl<Handler, Inner: Brush> Deref for Handle<Handler, Inner> {
    type Target = Inner;
    fn deref(&self) -> &Inner {
        &self.inner
    }
}

impl<Handler, Inner: Brush> DerefMut for Handle<Handler, Inner> {
    fn deref_mut(&mut self) -> &mut Inner {
        &mut self.inner
    }
}

impl<Handler, Inner: Brush> Handle<Handler, Inner> {
    pub fn new(inner: Inner) -> Self {
        Self {
            inner,
            handler: PhantomData,
        }
    }

    pub fn inner(&mut self) -> &mut Inner {
        &mut self.inner
    }
}

enum Mirrored {}
enum Rotated<const _1024THS: i16> {}
enum ScaledUp<const FACTOR: u32> {}
enum ScaledDown<const FACTOR: u32> {}

trait BrushMoves: Brush + Sized {
    fn left_turn(&mut self) -> &mut Self {
        self.stroke(0.5).rotate(0.25).stroke(0.5)
    }

    fn right_turn(&mut self) -> &mut Self {
        self.mirrored().left_turn().inner()
    }
}

impl<AnyBrush: Brush> BrushExt for AnyBrush {}
trait BrushExt: Brush + Sized {
    fn mirrored(self) -> Handle<Mirrored, Self> {
        Handle::new(self)
    }

    fn scaled_up<const UP: u32>(self) -> Handle<ScaledUp<UP>, Self> {
        Handle::new(self)
    }

    fn scaled_down<const DOWN: u32>(self) -> Handle<ScaledDown<DOWN>, Self> {
        Handle::new(self)
    }

    fn scaled_by<const NUMERATOR: u32, const DENOMINATOR: u32>(self) -> Handle<ScaledDown<DENOMINATOR>, Handle<ScaledUp<NUMERATOR>, Self>> {
        self.scaled_up::<NUMERATOR>().scaled_down::<DENOMINATOR>()
    }
}

impl<Inner: Brush> Brush for Handle<Mirrored, Inner> {
    fn stroke(&mut self, scale: f64) -> &mut Self {
        self.inner.stroke(scale);
        self
    }

    fn rotate(&mut self, taus: f64) -> &mut Self {
        self.inner.rotate(-taus);
        self
    }
}


impl<Inner: Brush, const FACTOR: u32> Brush for Handle<ScaledUp<FACTOR>, Inner> {
    fn stroke(&mut self, scale: f64) -> &mut Self {
        self.inner.stroke(scale * (FACTOR as f64));
        self
    }

    fn rotate(&mut self, taus: f64) -> &mut Self {
        self.inner.rotate(taus);
        self
    }
}


impl<Inner: Brush, const FACTOR: u32> Brush for Handle<ScaledDown<FACTOR>, Inner> {
    fn stroke(&mut self, scale: f64) -> &mut Self {
        self.inner.stroke(scale / (FACTOR as f64));
        self
    }

    fn rotate(&mut self, taus: f64) -> &mut Self {
        self.inner.rotate(taus);
        self
    }
}
