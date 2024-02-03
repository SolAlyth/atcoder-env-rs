/// モノイドであることを表す。
pub trait Monoid {
    type Set: Clone;
    fn ident(&self) -> Self::Set;
    fn op(&self, lhs: &Self::Set, rhs: &Self::Set) -> Self::Set;
}
