use std::{collections::VecDeque, fmt::Debug};

pub struct FoldableDeque<'a, T: Clone> {
    elem: VecDeque<T>,
    front: Vec<T>,
    back: Vec<T>,
    e: T,
    f: Box<dyn Fn(&T, &T) -> T + 'a>
}

impl<'a, T: Clone> FoldableDeque<'a, T> {
    pub fn new(e: T, f: impl Fn(&T, &T) -> T + 'a) -> Self {
        Self { elem: VecDeque::new(), front: vec![], back: vec![], e, f: Box::from(f) }
    }
    
    pub fn len(&self) -> usize { self.elem.len() }
    
    fn acc_front(&self) -> &T { self.front.last().unwrap_or(&self.e) }
    fn acc_back(&self) -> &T { self.back.last().unwrap_or(&self.e) }
    
    pub fn fold(&self) -> T { (self.f)(self.acc_front(), self.acc_back()) }
    
    pub fn push_front(&mut self, v: T) {
        self.front.push((self.f)(&v, self.acc_front()));
        self.elem.push_front(v);
    }
    pub fn push_back(&mut self, v: T) {
        self.back.push((self.f)(self.acc_back(), &v));
        self.elem.push_back(v);
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let res = self.elem.pop_front();
        if self.front.pop().is_none() { self.recalc_acc(); }
        res
    }
    
    pub fn pop_back(&mut self) -> Option<T> {
        let res = self.elem.pop_back();
        if self.back.pop().is_none() { self.recalc_acc(); }
        res
    }
    
    fn recalc_acc(&mut self) {
        self.front.clear(); self.back.clear();
        let len = self.elem.len();
        for i in (0..len/2).rev() { self.front.push((self.f)(&self.elem[i], self.acc_front())); }
        for i in len/2..len { self.back.push((self.f)(self.acc_back(), &self.elem[i])); }
    }
}

impl<'a, T: Clone + Debug> Debug for FoldableDeque<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut front = self.front.clone();
        front.reverse();
        write!(f, "elem = {:?}\nfold = {:?} {:?}", self.elem, front, self.back)
    }
}
