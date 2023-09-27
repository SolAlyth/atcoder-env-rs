#![allow(non_upper_case_globals)]

use crate::{println, swap};

use std::{fmt::Display, ops::{Shl, Not}, cell::UnsafeCell, mem::transmute};

pub struct Solver<const sp: bool> { v: UnsafeCell<String>, b: bool, bf: UnsafeCell<bool> }
impl<const sp: bool> Solver<sp> {
    pub fn new(b: bool) -> Solver<true> {Solver::<true>{v:String::new().into(),b,bf:true.into()}}
    pub fn swapbf(&self, mut v: bool) -> bool {unsafe{swap(&mut*self.bf.get(),&mut v)}v}
    pub fn push(&self, v: &str) {unsafe{let s=&mut*self.v.get();if (sp||self.swapbf(sp))&&s.len()!=0{s.push(' ');}s.push_str(v);}}
    pub fn print(&self) {unsafe{let s=&mut*self.v.get();if s.len()!=0{println!("{}", s);s.clear();}}}
}
impl<T: Display, const sp: bool> Shl<T> for &Solver<sp> {type Output=Self; fn shl(self,rhs:T)->Self::Output{self.push(&format!("{}",rhs)); self}}
#[allow(non_camel_case_types)] pub struct end;
impl<const sp: bool> Shl<end> for &Solver<sp> {type Output=(); fn shl(self,_:end)->Self::Output{self.swapbf(true);if cfg!(debug_assertions)||self.b{self.print();}()}}
impl<'a> Not for &'a Solver<true> {type Output = &'a Solver<false>; fn not(self) -> Self::Output {unsafe{transmute(self)}}}
