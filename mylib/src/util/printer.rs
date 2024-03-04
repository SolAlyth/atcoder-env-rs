#![allow(non_camel_case_types, non_upper_case_globals)]

use std::{ops::{Shl, Not}, cell::{UnsafeCell, Cell}, mem::transmute, fmt::Write};
use crate::pr;

/// `<<` による出力を可能にするための構造体。
/// 
/// # Printable types
/// 
/// + `integers/floats`, `char`, `&str`, `&String`: `Display` のまま出力。
/// + `bool`: `"Yes" / "No"` を出力。
/// + `&[T]`: 要素を順に出力。間の空白は `sp` に従う。
pub struct Printer<const sp: bool = true> {
    buf: UnsafeCell<String>,
    endf: EndFlag,
    prvf: Cell<PreviousFlag>
}

#[derive(PartialEq, Eq)]
pub enum EndFlag {
    /// `Printer << end;` しても何もしません。用途無い。
    DoNothing,
    /// `Printer << end;` するたびに改行を挿入します。出力はしません。
    LineFeed,
    /// `Printer << end;` するたびに出力します。デバッグ・インタラクティブ問題向け。
    Print
}

use PreviousFlag::*;
#[derive(PartialEq, Eq, Clone, Copy)]
enum PreviousFlag {
    Space,
    NoSpace,
    LineHead,
}



impl Printer {
    pub fn new(endf: EndFlag) -> Self { Printer { buf: String::new().into(), endf, prvf: LineHead.into() } }
}

impl<const sp: bool> Printer<sp> {
    fn push(&self, v: impl PrinterDisplay) {
        unsafe {
            let buf = &mut *self.buf.get();
            let prvf = self.prvf.replace(if sp {Space} else {NoSpace});
            if (prvf == Space || sp) && prvf != LineHead { *buf += " "; }
            v.pdisp(sp, buf);
        }
    }
    
    pub fn print(&self) {
        unsafe {
            let buf = &mut *self.buf.get();
            let prvf = self.prvf.replace(LineHead);
            if prvf == LineHead { buf.pop(); }
            pr!("{buf}");
            buf.clear();
        }
    }
}

/// `Printer << value` の実装 ( [Printer::push] を呼び出すだけ)
impl<T: PrinterDisplay, const sp: bool> Shl<T> for &Printer<sp> { type Output = Self; fn shl(self, rhs: T) -> Self::Output { self.push(rhs); self } }

// !Printer<sp = true> で Printer<sp = false> にする (by transmute)
impl<'a> Not for &'a Printer<true> { type Output = &'a Printer<false>; fn not(self) -> Self::Output { unsafe { transmute(self) } } }



pub struct end;
// `Printer << end` の実装
impl<const sp: bool> Shl<end> for &Printer<sp> {
    type Output = ();
    fn shl(self, _: end) -> Self::Output {
        if self.endf == EndFlag::LineFeed { self.prvf.replace(LineHead); self.push('\n'); }
        else if self.endf == EndFlag::Print { self.print(); }
    }
}



/// `Printer << var` で表示可能であることを表す。
trait PrinterDisplay { fn pdisp(&self, sp: bool, buf: &mut String); }

/// `PrinterDisplay` を `Display` に fallback させる
macro_rules! fall { ($($t:ty),+) => { $( impl PrinterDisplay for $t { fn pdisp(&self, _: bool, buf: &mut String) { write!(buf, "{self}").unwrap(); } } )+ }; }
fall!( u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64, char, &str, &String );

impl PrinterDisplay for bool { fn pdisp(&self, _: bool, buf: &mut String) { *buf += if *self {"Yes"} else{ "No" }; }}

impl<T: PrinterDisplay> PrinterDisplay for &[T] {
    fn pdisp(&self, sp: bool, buf: &mut String) {
        for e in *self { e.pdisp(sp, buf); if sp { *buf += " "; } }
        if sp && !self.is_empty() { buf.pop(); }
    }
}
