pub trait Update: Sized + PartialOrd {
    fn update_max(&mut self, value: Self) { if (self as &Self).partial_cmp(&value).unwrap().is_lt() { *self = value; } }
    fn update_min(&mut self, value: Self) { if (self as &Self).partial_cmp(&value).unwrap().is_gt() { *self = value; } }
}

macro_rules! impl_update {
    ($($t:ty);+) => { $( impl Update for $t {} )+ };
}

impl_update!(u8; u16; u32; u64; u128; i8; i16; i32; i64; i128; f32; f64);



pub trait CharFn: Copy {
    fn add(self, v: isize) -> Self;
    fn to_lower(self) -> Self;
    fn to_upper(self) -> Self;
    fn lower_to_us(self) -> usize;
    fn upper_to_us(self) -> usize;
    fn num_to_us(self) -> usize;
}

// '0' == 48
// 'A' == 65
// 'a' == 97

impl CharFn for char {
    fn add(self, v: isize) -> Self { (self as isize + v) as u8 as char }
    fn to_lower(self) -> Self { self.add(32) }
    fn to_upper(self) -> Self { self.add(-32) }
    fn lower_to_us(self) -> usize { self as usize - 97 }
    fn upper_to_us(self) -> usize { self as usize - 65 }
    fn num_to_us(self) -> usize { self as usize - 48 }
}
