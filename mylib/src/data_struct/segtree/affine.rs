use super::{Monoid, Map};
use crate::mylib::Mint;

#[derive(Clone)]
pub struct AffineNode { pub sum: Mint, pub len: usize }

pub struct Sum;

impl Monoid for Sum {
    type Set = AffineNode;
    fn e() -> Self::Set { AffineNode { sum: 0.into(), len: 0 } }
    fn op(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        AffineNode { sum: lhs.sum + rhs.sum, len: lhs.len + rhs.len }
    }
}

pub struct Affine;

impl Map<AffineNode> for Affine {
    type F = (Mint, Mint);
    fn id() -> Self::F { (1.into(), 0.into()) }
    fn comp(f: &Self::F, g: &mut Self::F) { g.0 += f.0; g.1 += f.1; }
    fn map(f: &Self::F, x: &mut AffineNode) {
        x.sum += f.0 * x.len + f.1;
    }
}
