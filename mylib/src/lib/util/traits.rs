pub trait AssignMinMax: Sized + PartialOrd {
    fn assign_max(&mut self, value: Self) { if (self as &Self).partial_cmp(&value).unwrap().is_lt() { *self = value; } }
    fn assign_min(&mut self, value: Self) { if (self as &Self).partial_cmp(&value).unwrap().is_gt() { *self = value; } }
}

macro_rules! impl_update {
    ($($t:ty);+) => { $( impl AssignMinMax for $t {} )+ };
}

impl_update!(u8; u16; u32; u64; u128; i8; i16; i32; i64; i128; f32; f64);



pub trait CharFn: Copy {
    fn add(self, v: isize) -> Self;
    fn to_lower(self) -> Self;
    fn to_upper(self) -> Self;
    fn lower_to_us(self) -> usize;
    fn upper_to_us(self) -> usize;
    fn num_to_us(self) -> usize;
    fn into_lower(v: usize) -> Self;
    fn into_upper(v: usize) -> Self;
}

impl CharFn for char {
    fn add(self, v: isize) -> Self { (self as isize + v) as u8 as char }
    fn to_lower(self) -> Self { self.add(32) }
    fn to_upper(self) -> Self { self.add(-32) }
    fn lower_to_us(self) -> usize { self as usize - 97 }
    fn upper_to_us(self) -> usize { self as usize - 65 }
    fn num_to_us(self) -> usize { self as usize - 48 }
    fn into_lower(v: usize) -> Self { (v+97) as u8 as char }
    fn into_upper(v: usize) -> Self { (v+65) as u8 as char }
}
