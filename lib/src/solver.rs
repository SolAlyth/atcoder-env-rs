use crate::println;

use std::{fmt::Display, ops::Shl, cell::UnsafeCell};

pub struct Solver { v: UnsafeCell<String>, b: bool }
impl Solver {
    pub fn new(b: bool) -> Self {Solver{v:String::new().into(),b}}
    pub fn push(&self, v: &str) {unsafe{let s=&mut*self.v.get();if s.len()!=0{s.push(' ');}s.push_str(v);}}
    pub fn print(&self) {unsafe{let s=&mut*self.v.get();if s.len()!=0{println!("{}", s);s.clear();}}}
}
impl<T: Display> Shl<T> for &Solver {type Output=Self; fn shl(self,rhs:T)->Self::Output{self.push(&format!("{}",rhs)); self}}
#[allow(non_camel_case_types)] pub struct end;
impl Shl<end> for &Solver {type Output=(); fn shl(self,_:end)->Self::Output{if cfg!(debug_assertions)||self.b{self.print();}()}}
