use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub struct HexU16(u16);

impl From<u16> for HexU16 { fn from(from: u16) -> Self { Self(from) } }
impl From<HexU16> for u16 { fn from(from: HexU16) -> Self { from.0 } }

impl Debug for HexU16 { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:#06X}", self.0) } }