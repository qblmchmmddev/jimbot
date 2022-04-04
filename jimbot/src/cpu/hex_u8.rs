use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub struct HexU8(u8);

impl From<u8> for HexU8 { fn from(from: u8) -> Self { Self(from) } }
impl From<HexU8> for u8 { fn from(from: HexU8) -> Self { from.0 } }

impl Debug for HexU8 { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:#04X}", self.0) } }