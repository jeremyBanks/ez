use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

fn main() {
    let mut b = DebugPen;
    b.stroke();

    eprintln!("Scaled up:");
    let mut b = b.scaled_up::<2>();
    b.stroke();
    b.rotate_by(0.5);

    {
        eprintln!("Mirrored");
        let mut b = b.mirrored();
        b.stroke();
        b.rotate_by(0.5);
    }

    eprintln!("Down more:");
    let mut b = b.scaled_down::<3>();
    b.stroke();
    b.rotate_by(0.5);

    eprintln!("Resetting to root brush");
    let mut b = b.reset();
    b.stroke();
}

trait Pen: Sized {
    fn stroke_by(&mut self, length: f64) -> &mut Self;

    fn rotate_by(&mut self, taus: f64) -> &mut Self;

    fn stroke(&mut self) -> &mut Self {
        self.stroke_by(1.0)
    }

    fn turn_by(&mut self, taus: f64) -> &mut Self {
        self.scaled_down::<2>().stroke().rotate_by(taus).stroke().pop()
    }

    fn left_turn(&mut self) -> &mut Self {
        self.turn_by(-0.25)
    }

    fn right_turn(&mut self) -> &mut Self {
        self.turn_by(0.25)
    }

    fn left_square(&mut self) -> &mut Self {
        self.left_turn().left_turn().left_turn().left_turn()
    }

    fn right_square(&mut self) -> &mut Self {
        self.left_turn().left_turn().left_turn().left_turn()
    }
}

trait ResetPen: Pen {
    fn reset(&mut self) -> &mut Self {
        self
    }
}

#[derive(Debug)]
struct Tool<Behavior, Inner: Pen> {
    behavior: PhantomData<Behavior>,
    inner: Inner,
}

#[derive(Debug)]
struct DebugPen;

impl ResetPen for DebugPen {}
impl Pen for DebugPen {
    fn stroke_by(&mut self, length: f64) -> &mut Self {
        eprintln!("stroke_by({length})");
        self
    }

    fn rotate_by(&mut self, taus: f64) -> &mut Self {
        eprintln!("rotate_by({taus})");
        self
    }
}

impl<Behavior, Inner: Pen> Deref for Tool<Behavior, Inner> {
    type Target = Inner;
    fn deref(&self) -> &Inner {
        &self.inner
    }
}

impl<Behavior, Inner: Pen> DerefMut for Tool<Behavior, Inner> {
    fn deref_mut(&mut self) -> &mut Inner {
        &mut self.inner
    }
}

impl<Behavior, Inner: Pen> Tool<Behavior, Inner> {
    pub fn new(inner: Inner) -> Self {
        Self { inner, behavior: PhantomData }
    }

    pub fn pop(&mut self) -> &mut Inner {
        &mut self.inner
    }
}

enum Mirrored {}
enum ScaledUp<const FACTOR: u32> {}
enum ScaledDown<const FACTOR: u32> {}

trait PenExt: Pen {
    fn equip<Behavior>(self) -> Tool<Behavior, Self> {
        Tool::new(self)
    }

    fn mirrored(self) -> Tool<Mirrored, Self> {
        self.equip()
    }

    fn scaled_up<const UP: u32>(self) -> Tool<ScaledUp<UP>, Self> {
        self.equip()
    }

    fn scaled_down<const DOWN: u32>(self) -> Tool<ScaledDown<DOWN>, Self> {
        self.equip()
    }

    fn scaled_by<const NUMERATOR: u32, const DENOMINATOR: u32>(
        self,
    ) -> Tool<ScaledDown<DENOMINATOR>, Tool<ScaledUp<NUMERATOR>, Self>> {
        self.equip().equip()
    }
}
impl<AnyPen: Pen> PenExt for AnyPen {}

impl<Inner: Pen> Pen for Tool<Mirrored, Inner> {
    fn stroke(&mut self) -> &mut Self {
        self.inner.stroke(scale);
        self
    }

    fn rotate_by(&mut self, taus: f64) -> &mut Self {
        self.inner.rotate_by(-taus);
        self
    }
}

impl<Inner: Pen, const FACTOR: u32> Pen for Tool<ScaledUp<FACTOR>, Inner> {
    fn stroke(&mut self) -> &mut Self {
        self.inner.stroke(scale * (FACTOR as f64));
        self
    }

    fn rotate_by(&mut self, taus: f64) -> &mut Self {
        self.inner.rotate_by(taus);
        self
    }
}

impl<Inner: Pen, const FACTOR: u32> Pen for Tool<ScaledDown<FACTOR>, Inner> {
    fn stroke(&mut self) -> &mut Self {
        self.inner.stroke(scale / (FACTOR as f64));
        self
    }

    fn rotate_by(&mut self, taus: f64) -> &mut Self {
        self.inner.rotate_by(taus);
        self
    }
}
