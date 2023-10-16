pub trait Bit {
    fn get(&self, n: usize) -> bool;
    fn set_true(&mut self, n: usize);
    fn set_false(&mut self, n: usize);
}

impl Bit for usize {
    fn get(&self, n: usize) -> bool { self>>n&1==1 }
    fn set_true(&mut self, n: usize) { *self|=1<<n; }
    fn set_false(&mut self, n: usize) { *self&=!(1<<n); }
}
