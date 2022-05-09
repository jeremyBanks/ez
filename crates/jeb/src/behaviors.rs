use crate::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TurnsFromArms;
impl MetaBrushBehavior for TurnsFromArms {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TurnsFromCurves;
impl MetaBrushBehavior for TurnsFromCurves {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct RoundedTurns(pub Ratio);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Scaled(pub Ratio);

impl MetaBrushBehavior for Scaled {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ArmsFromStrokes;
impl MetaBrushBehavior for ArmsFromStrokes {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Mirrored;
impl MetaBrushBehavior for Mirrored {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ZigZagStrokes;
impl MetaBrushBehavior for ZigZagStrokes {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct CurveyStrokes;
impl MetaBrushBehavior for CurveyStrokes {}
