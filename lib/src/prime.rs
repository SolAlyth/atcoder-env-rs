pub trait PrimeType: Copy {
    fn to(self) -> usize;
}

macro_rules! pt {($($t:ty),*)=>{$(
    impl PrimeType for $t {
        fn to(self) -> usize {self as usize}
    }
)*}}
pt!(usize,u32,u64,u128);

pub struct PrimeList<T: PrimeType> { l: Vec<T>, s: Vec<bool>, m: usize, n: usize }

impl<T: PrimeType> PrimeList<T> {
    const MAX: usize = 10000000;
    
    pub fn new() -> Self {Self{l:vec![],s:vec![false;Self::MAX+1],m:Self::MAX,n:1}}
    pub fn next_sieve(&mut self)->bool{if self.m<=self.n{return false;} }
}
