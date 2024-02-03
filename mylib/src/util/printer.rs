#![allow(non_camel_case_types, non_upper_case_globals)]

use {
    std::{ops::{Shl, Not}, cell::{UnsafeCell, Cell}, mem::transmute},
    itertools::Itertools
};



#[macro_export]
macro_rules! pr {
    ($($args:tt)*) => {
        if !$crate::SUBMISSION { print!("\x1b[32m{}\x1b[0m", format!($($args)*).split('\n').map(|s| format!(">> {s}\n")).reduce(|acc,s| acc+&s).unwrap()); }
        if $crate::SUBMISSION { println!($($args)*); }
    }
}

#[macro_export]
macro_rules! epr {
    ($($args:tt)*) => {
        if !$crate::SUBMISSION { print!("\x1b[31m{}\x1b[0m", format!($($args)*).split('\n').map(|s| format!(">> {s}\n")).reduce(|acc,s| acc+&s).unwrap()); }
        if $crate::SUBMISSION { /* do nothing */ }
    }
}



/// `<<` による出力を可能にするための構造体。
///
/// # Note
///
/// |name|type|description|
/// |-|-|-|
/// |`sp`|`bool`|スペースを入れるかどうか|
/// |`bsp`|`Cell<bool>`|一つ前の `sp`|
/// |`endf`|`bool`|`Printer << end` で出力するかどうか|
/// |`out`|`UnsafeCell<String>`|出力待ち文字列|
///
/// # Behavior
///
/// 大まかな動作だけ。この通りに実装すると一部表示が気持ち悪くなるなど。
///
/// ## when `Printer << value`
///
/// `bsp == sp == false` のとき、スペースを入れない。そうでないとき、スペースを入れる。
/// その後 `value` を `PrinterDisplay` に従って追加する。
///
/// ## when `Printer << end`
///
/// `bsp` を `true` にする。
/// `endf == false` なら出力しない。そうでないなら `out` を出力して空にする。
///
/// ## when `!Printer<true>`
///
/// `transmute` で `Printer<true> -> Printer<false>` にする。
///
/// ## when `Printer.print()`
///
/// `out` を出力して空にする。
pub struct Printer<const sp: bool = true> {
    out: UnsafeCell<String>,
    endf: bool,
    bsp: Cell<bool>
}

impl Printer {
    pub fn new(endf: bool) -> Self {
        Printer { out: String::new().into(), endf, bsp: true.into() }
    }
}

impl<const sp: bool> Printer<sp> {
    fn push(&self, v: &str) {
        unsafe {
            let s = &mut *self.out.get();
            // Cell::replace は副作用が付くので順番に注意
            if (self.bsp.replace(sp) || sp) && !s.is_empty() { *s += " "; }
            *s += v;
        }
    }
    
    pub fn print(&self) {
        unsafe {
            let s = &mut *self.out.get();
            if !s.is_empty() { pr!("{}", s); s.clear(); }
        }
    }
}

/// `Printer << value` の実装 ( [PrinterDisplay::pdisp] を呼び出すだけ)
impl<T: PrinterDisplay, const sp: bool> Shl<T> for &Printer<sp> { type Output = Self; fn shl(self, rhs: T) -> Self::Output { self.push(&rhs.pdisp(sp)); self } }

// !Printer<sp = true> で Printer<sp = false> にする (by transmute)
impl<'a> Not for &'a Printer<true> { type Output = &'a Printer<false>; fn not(self) -> Self::Output { unsafe { transmute(self) } } }



pub struct end;
// `Printer << end` の実装
impl<const sp: bool> Shl<end> for &Printer<sp> { type Output = (); fn shl(self, _: end) -> Self::Output { self.bsp.replace(true); if self.endf { self.print(); } } }



/// `Printer << var` で表示可能
trait PrinterDisplay { fn pdisp(&self, sp: bool) -> String; }

/// marker trait
trait PrimitivePrinterDisplay: PrinterDisplay {}

/// Display を PrimitivePrinterDisplay に fallback させる
macro_rules! fall { ($($t:ty);+) => { $( impl PrinterDisplay for $t { fn pdisp(&self, _: bool) -> String { format!("{}", self) } } impl PrimitivePrinterDisplay for $t {} )+ }; }

fall!(
    u8; u16; u32; u64; u128; usize;
    i8; i16; i32; i64; i128; isize;
    f32; f64;
    char; &str; &String; String
);

impl PrinterDisplay for bool {
    fn pdisp(&self, _: bool) -> String { String::from(if *self {"Yes"} else {"No"}) }
}

impl PrimitivePrinterDisplay for bool {}

/// Vec<T: Primitive>
impl<T: PrimitivePrinterDisplay> PrinterDisplay for Vec<T> {
    fn pdisp(&self, sp: bool) -> String {
        self.iter().map(|v| v.pdisp(sp)).join(if sp {" "} else {""})
    }
}

/// &[T: Primitive]
impl<T: PrimitivePrinterDisplay> PrinterDisplay for &[T] {
    fn pdisp(&self, sp: bool) -> String {
        self.iter().map(|v| v.pdisp(sp)).join(if sp {" "} else {""})
    }
}

#[cfg(test)]
mod test {
    #[allow(unused)] use super::*;
    
    #[test]
    fn a() {
        #![allow(unused_must_use)]
        
        let out = Printer::new(true);
        
        &out << String::from("aiueo");
    }
}
