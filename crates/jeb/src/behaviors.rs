use crate::*;

pub struct TurnsFromArms;
impl crate::metabrush::MetaBrushBehavior for TurnsFromArms {}

pub struct TurnsFromCurves;
impl crate::metabrush::MetaBrushBehavior for TurnsFromCurves {}

#[derive(Clone, Copy, Debug)]
pub struct RoundedTurns(Ratio);

#[derive(Clone, Copy, Debug)]
pub struct Scaled(Ratio);

impl crate::metabrush::MetaBrushBehavior for Scaled {}

pub struct ArmsFromStrokes;
impl crate::metabrush::MetaBrushBehavior for ArmsFromStrokes {}

pub struct ZigZagStrokes;
impl crate::metabrush::MetaBrushBehavior for ZigZagStrokes {}

pub struct CurveyStrokes;
impl crate::metabrush::MetaBrushBehavior for CurveyStrokes {}
