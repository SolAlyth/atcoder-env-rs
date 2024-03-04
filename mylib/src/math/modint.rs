// use ac_libary;

use std::{marker::PhantomData, ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg, Deref}};
use proconio::source::Readable;

/// ModInt の法を表す。
pub trait Modulus: Sized + Copy {
    const VALUE: u64;
    fn inv(value: ModInt<Self>) -> ModInt<Self>;
}

pub mod base {
    use super::{Modulus, ModInt};
    
    #[derive(Clone, Copy)] pub struct Mod998244353;
    impl Modulus for Mod998244353 {
        const VALUE: u64 = 998244353;
        fn inv(value: ModInt<Self>) -> ModInt<Self> { value.pow(Self::VALUE - 2) }
    }
    
    #[derive(Clone, Copy)] pub struct Mod1000000007;
    impl Modulus for Mod1000000007 {
        const VALUE: u64 = 1000000007;
        fn inv(value: ModInt<Self>) -> ModInt<Self> { value.pow(Self::VALUE - 2) }
    }
}




/// static な法の ModInt
/// 
/// # Guarantee
/// 
/// `value` は `[0, M)` に含まれる数である。
#[derive(Clone, Copy)]
pub struct ModInt<M: Modulus> { value: u64, ph: PhantomData<M> }

impl<M: Modulus> ModInt<M> {
    pub const fn new(value: i64) -> Self {
        let tmp = value.abs() as u64 % M::VALUE;
        let value = if 0 <= value { tmp } else { M::VALUE - tmp };
        ModInt { value, ph: PhantomData }
    }
    
    fn simplify(mut self) -> Self {
        if M::VALUE <= *self {
            if *self < 2*M::VALUE {
                self.value -= M::VALUE;
            } else {
                self.value %= M::VALUE;
            }
        }
        self
    }
    
    fn _add(mut self, r: Self) -> Self { self.value += *r; self.simplify() }
    fn _neg(mut self) -> Self { if *self != 0 { self.value = M::VALUE - *self; } self }
    fn _mul(mut self, r: Self) -> Self { self.value *= *r; self.simplify() }
    
    pub fn pow(mut self, mut exp: u64) -> Self {
        let mut out = 1.into();
        while exp != 0 {
            if exp % 2 == 1 { out *= self; }
            self *= self; exp /= 2;
        }
        out
    }
}

impl<M: Modulus> From<i64> for ModInt<M> { fn from(value: i64) -> Self { Self::new(value) } }

impl<M: Modulus> Add for ModInt<M> { type Output = Self; fn add(self, rhs: Self) -> Self::Output { self._add(rhs) } }
impl<M: Modulus> AddAssign for ModInt<M> { fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl<M: Modulus> Neg for ModInt<M> { type Output = Self; fn neg(self) -> Self::Output { self._neg() } }
impl<M: Modulus> Sub for ModInt<M> { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { self + -rhs } }
impl<M: Modulus> SubAssign for ModInt<M> { fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
impl<M: Modulus> Mul for ModInt<M> { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { self._mul(rhs) } }
impl<M: Modulus> MulAssign for ModInt<M> { fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; } }
impl<M: Modulus> Deref for ModInt<M> { type Target = u64; fn deref(&self) -> &Self::Target { &self.value } }

impl<M: Modulus> Readable for ModInt<M> { type Output = Self; fn read<R: std::io::prelude::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output { ModInt::new(source.next_token_unwrap().parse().unwrap()) } }
