/// 結合律 `(ab)c == a(bc)` を満たし、単位元 `e` を持つ二項演算
pub trait Monoid {
    const NOP: bool = false;
    
    type Set: Clone;
    
    /// 単位元
    fn e() -> Self::Set;
    /// 演算 `lhs * rhs`
    fn op(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set;
    
    fn debug(#[allow(unused_variables)] v: &Self::Set) -> String { "no info".into() }
}

/// 合成可能で、恒等写像 `id` を持つ写像
pub trait Map<MnSet: Clone, const NOP: bool = false> {
    const NOP: bool = false;
    
    type F: Clone;
    
    /// 恒等写像
    fn id() -> Self::F;
    /// 写像の合成 `f*g == (x -> f(g(x)))`
    fn comp(f: &Self::F, g: &mut Self::F);
    /// 写像の適用 `x -> f(x)`
    fn map(f: &Self::F, x: &mut MnSet);
    
    /* fn comp_pow(mut f: Self::F, mut n: usize) -> Self::F {
        let mut out = self.id();
        while n != 0 {
            if n&1 == 1 { out = self.comp(&f, &out); }
            f = self.comp(&f, &f);
            n >>= 1;
        }
        out
    } */
    
    fn debug(#[allow(unused_variables)] f: &Self::F) -> String { "no info".into() }
}
