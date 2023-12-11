#![allow(non_camel_case_types, non_upper_case_globals)]

use {
    std::{ops::{Shl, Not}, cell::UnsafeCell, mem::{transmute, swap}},
    itertools::Itertools
};

/// Shl `<<` による出力を可能にする
pub struct Printer<const sp: bool = true> {
    /// 出力待ち文字列
    v: UnsafeCell<String>,
    /// `Printer << end` で出力するかのフラグ
    endf: bool,
    /// `!Printer << var` で空白を追加するかのフラグ
    spf: UnsafeCell<bool>
}

impl Printer {
    pub fn new(endf: bool) -> Self {
        Printer { v: String::new().into(), endf, spf: true.into() }
    }
}

impl<const sp: bool> Printer<sp> {
    /// 前の `spf` を取り出し、`f` を設定
    fn swap_spf(&self, mut f: bool) -> bool {
        unsafe { swap(&mut *self.spf.get(), &mut f) }
        f
    }
    
    /// 出力待ち文字列を追加
    fn push(&self, v: &str) {
        unsafe {
            let s = &mut *self.v.get();
            // swap_spf が副作用を持つため、lazy 評価させない
            if (self.swap_spf(sp) || sp) && !s.is_empty() {
                *s += " ";
            }
            *s += v;
        }
    }
    
    pub fn print(&self) {
        unsafe {
            let s = &mut *self.v.get();
            if !s.is_empty() {
                crate::pr!("{}", s); s.clear();
            }
        }
    }
}

// << による出力の実装
impl<T: PrinterDisplay, const sp: bool> Shl<T> for &Printer<sp> {
    type Output = Self;
    fn shl(self, rhs: T) -> Self::Output {
        self.push(&rhs.pdisp(sp)); self
    }
}

// !Printer<sp = true> で Printer<sp = false> にする
impl<'a> Not for &'a Printer<true> {
    type Output = &'a Printer<false>;
    fn not(self) -> Self::Output { unsafe { transmute(self) } }
}



pub struct end;

impl<const sp: bool> Shl<end> for &Printer<sp> {
    type Output=();
    fn shl(self, _:end) -> Self::Output {
        self.swap_spf(true);
        if !crate::SUBMISSION || self.endf {
            self.print();
        }
    }
}



/// `Printer << var` で表示可能
trait PrinterDisplay {
    fn pdisp(&self, sp: bool) -> String;
}

/// [PrinterDisplay] の最小型
trait PrimitivePrinterDisplay: PrinterDisplay {}

// PrimitivePrinterDisplay の実装 (Display に fallback)
macro_rules! fall {
    ($($t:ty);+) => {
        $(
            impl PrinterDisplay for $t {
                fn pdisp(&self, _: bool) -> String { format!("{}", self) }
            }
            impl PrimitivePrinterDisplay for $t {}
        )+
    };
}

fall!(
    u8; u16; u32; u64; u128; usize;
    i8; i16; i32; i64; i128; isize;
    f32; f64;
    char; &str; String
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
