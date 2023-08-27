pub trait Bit { fn nth_bit(self, n: usize) -> Self; fn add_nth_bit(self, n: usize) -> Self; }
impl Bit for usize { fn nth_bit(self,n:usize)->Self{self>>n&1} fn add_nth_bit(self,n:usize)->Self{self|(1<<n)}}
