pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

///
/// Empty struct AKA tag to identify player
/// No data is held
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player ;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy ;