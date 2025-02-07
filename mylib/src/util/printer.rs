#![allow(non_camel_case_types)]

use std::{mem::{replace, transmute}, ops::{Not, Shl}, sync::{Mutex, MutexGuard, OnceLock}};



static INTERNAL: OnceLock<Mutex<Internal>> = OnceLock::new();
pub static out: Printer = Printer(&INTERNAL);



pub struct Internal {
    buf: String,
    endf: EndFlag,
    prvf: PreviousFlag
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

pub struct end;


#[derive(Clone, Copy)]
pub struct Printer<const sp: bool = true>(&'static OnceLock<Mutex<Internal>>);

impl<const sp: bool> Printer<sp> {
    pub fn init(&self, endf: EndFlag) {
        let is_err = self.0.set(Mutex::new(Internal { buf: String::new(), endf, prvf: LineHead })).is_err();
        if is_err { panic!("[@printer] Error: Second call of Printer::init"); }
    }
    
    fn get(&self) -> MutexGuard<Internal> { self.0.get().unwrap().lock().unwrap() }
    fn push(&self, v: impl PrinterDisplay) { self.get().push(v, sp); }
    pub fn print(&self) { self.get().print(); }
}

impl Internal {
    fn push(&mut self, v: impl PrinterDisplay, sp: bool) {
        let prvf = replace(&mut self.prvf, if sp {Space} else {NoSpace});
        let buf = &mut self.buf;
        if prvf != LineHead && (prvf == Space || sp) { *buf += " "; }
        v.pdisp(sp, buf);
    }
    
    fn print(&mut self) {
        let prvf = replace(&mut self.prvf, LineHead);
        let buf = &mut self.buf;
        if prvf == LineHead { buf.pop(); }
        
        if buf.is_empty() { return; }
        
        if crate::mylib::SUBMISSION {
            println!("{buf}");
        } else {
            eprint!("\x1b[32m");
            for (i, s) in buf.split('\n').enumerate() {
                eprint!("{}", if i == 0 {">> "} else {"   "});
                println!("{s}");
            }
            eprint!("\x1b[0m");
        }
        buf.clear();
    }
}

impl<T: PrinterDisplay, const sp: bool> Shl<T> for Printer<sp> { type Output = Self; fn shl(self, v: T) -> Self { self.push(v); self } }
impl Not for Printer<true> { type Output = Printer<false>; fn not(self) -> Printer<false> { unsafe { transmute(self) } } }

impl<const sp: bool> Shl<end> for Printer<sp> {
    type Output = Self;
    fn shl(self, _: end) -> Self {
        let mut itn = self.0.get().unwrap().lock().unwrap();
        use EndFlag::*;
        match itn.endf {
            Print => { itn.print(); }
            LineFeed => { itn.buf += "\n"; itn.prvf = LineHead; }
            DoNothing => {}
        }
        self
    }
}



/// `Printer << value` で表示可能であることを表す。
trait PrinterDisplay { fn pdisp(&self, sp: bool, buf: &mut String); }

/// `PrinterDisplay` を `Display` に fallback させる
macro_rules! fall { ($($t:ty),+) => { $( impl PrinterDisplay for $t { fn pdisp(&self, _: bool, buf: &mut String) { *buf += &format!("{self}"); } } )+ }; }
fall!( u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64/* , ac_library::ModInt998244353, ac_library::ModInt1000000007  */);

impl PrinterDisplay for char { fn pdisp(&self, _: bool, buf: &mut String) { buf.push(*self); } }
impl PrinterDisplay for &str { fn pdisp(&self, _: bool, buf: &mut String) { buf.push_str(self); } }
impl PrinterDisplay for &String { fn pdisp(&self, _: bool, buf: &mut String) { buf.push_str(self); } }
impl PrinterDisplay for bool { fn pdisp(&self, _: bool, buf: &mut String) { *buf += if *self {"Yes"} else{ "No" }; } }

impl<T: PrinterDisplay> PrinterDisplay for &[T] {
    fn pdisp(&self, sp: bool, buf: &mut String) {
        for e in *self { e.pdisp(sp, buf); if sp { *buf += " "; } }
        if sp && !self.is_empty() { buf.pop(); }
    }
}
