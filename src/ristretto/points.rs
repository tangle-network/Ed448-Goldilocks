#![allow(non_snake_case)]

use crate::curve::twedwards::extended::ExtendedPoint;
use subtle::{Choice, ConstantTimeEq};

/// The bytes representation of a compressed point.
pub type RistrettoPointBytes = [u8; 56];

#[derive(Copy, Clone, Debug)]
pub struct RistrettoPoint(pub(crate) ExtendedPoint);

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct CompressedRistretto(pub RistrettoPointBytes);

impl Default for CompressedRistretto {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl ConstantTimeEq for CompressedRistretto {
    fn ct_eq(&self, other: &CompressedRistretto) -> Choice {
        self.as_bytes().ct_eq(other.as_bytes())
    }
}

impl PartialEq for CompressedRistretto {
    fn eq(&self, other: &CompressedRistretto) -> bool {
        self.ct_eq(other).into()
    }
}

impl Eq for CompressedRistretto {}

impl CompressedRistretto {
    pub const IDENTITY: Self = Self([0u8; 56]);

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl RistrettoPoint {
    /// The generator of the Ristretto group.
    pub const GENERATOR: RistrettoPoint = RistrettoPoint(ExtendedPoint::GENERATOR);
    /// The identity element of the group: the point at infinity.
    pub const IDENTITY: RistrettoPoint = RistrettoPoint(ExtendedPoint::IDENTITY);

    pub fn equals(&self, other: &RistrettoPoint) -> bool {
        let XY = self.0.X * other.0.Y;
        let YX = self.0.Y * other.0.X;
        XY == YX
    }

    pub fn encode(&self) -> CompressedRistretto {
        todo!()
    }
}

impl CompressedRistretto {
    pub fn identity() -> CompressedRistretto {
        CompressedRistretto([0; 56])
    }

    pub fn decode(&self) -> Option<RistrettoPoint> {
        todo!()
    }
}