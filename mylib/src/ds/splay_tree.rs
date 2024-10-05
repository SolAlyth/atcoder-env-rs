use crate::mylib::util::traits::AsBound;

use std::cell::Cell;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Index, RangeBounds};
use std::ptr::{eq as ptr_eq, null_mut};
use std::mem::replace;



pub trait SplayOps {
    type Value: Sized + Clone;
    type Acc: Sized + Clone;
    type Lazy: Sized + Clone;
    
    /// 単頂点 `Value` を `Acc` 化する関数。
    fn to_acc(value: &Self::Value) -> Self::Acc;
    /// `Acc` のマージを行う関数。
    fn merge_acc(lhs: &Self::Acc, rhs: &Self::Acc) -> Self::Acc;
    /// 遅延作用 `Lazy` の反映を行う関数。`len` は `Acc` の対象となっている頂点数。
    fn act_lazy(value: &mut Self::Value, acc: &mut Self::Acc, lazy: &Self::Lazy, len: usize);
    /// 遅延作用 `Lazy` の合成を行う関数。
    fn comp_lazy(lazy: &mut Self::Lazy, ad: &Self::Lazy);
}



/// Splay 木
/// 
/// # 搭載トレイト
/// 
/// + `Clone`
/// + `Debug where Ops::Value: Debug`
/// + `FromIterator<Value>`
/// + `IntoIterator for &SplayTree`
/// + `Index<usize>`: `splay.get(i).unwrap()` と等価。
/// 
/// # 実装のメモ
/// 
/// `insert/delete` 系の破壊的な関数は `&mut self` で、`splay` しかしない関数は `&self` にしてある。
pub struct SplayTree<Ops: SplayOps> (Cell<*mut Node<Ops>>);

impl<Ops: SplayOps> SplayTree<Ops> {
    fn root(&self) -> *mut Node<Ops> { self.0.get() }
    
    pub fn new() -> Self { Self(Cell::new(null_mut())) }
    pub fn is_empty(&self) -> bool { self.root().is_null() }
    pub fn len(&self) -> usize { Node::get(self.root()).map_or(0, |root| root.len) }
    
    /// 指定した位置に挿入する。一番右なら `at = self.len` とすればよい。
    /// 
    /// # Panics
    /// 
    /// `!(at <= self.len)`
    pub fn insert(&mut self, at: usize, value: Ops::Value) {
        assert!(at <= self.len());
        let [l, r] = Node::split_at(self.root(), at);
        let node = Node::new(value);
        self.0.set(Node::merge_3(l, node, r));
    }
    
    /// 指定した位置を削除する。
    /// 
    /// # Panics
    /// 
    /// `!(at < self.len)`
    pub fn delete(&mut self, at: usize) -> Ops::Value {
        assert!(at < self.len());
        let (l, c, r) = Node::split_at_3(self.root(), at, at+1).unwrap();
        self.0.set(Node::merge(l, r));
        unsafe { Box::from_raw(c) }.value
    }
    
    /// `tree[range]` を逆順にする。
    pub fn reverse(&mut self, range: impl RangeBounds<usize>) {
        let [st, ed] = range.as_bounds(self.len());
        let Some((l, c, r)) = Node::split_at_3(self.root(), st, ed) else { return; };
        c.rev ^= true;
        c.push();
        self.0.set(Node::merge_3(l, c, r));
    }
    
    /// `tree[range]` を畳み込んだ値を返す。区間幅が `0` のとき `None` を返す。
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Option<Ops::Acc> {
        let [st, ed] = range.as_bounds(self.len());
        let Some((l, c, r)) = Node::split_at_3(self.root(), st, ed) else { return None; };
        c.update();
        let res = c.acc.clone();
        self.0.set(Node::merge_3(l, c, r));
        Some(res)
    }
    
    /// `tree[range]` に `lazy` を作用させる。
    pub fn act(&mut self, range: impl RangeBounds<usize>, lazy: Ops::Lazy) {
        let [st, ed] = range.as_bounds(self.len());
        let Some((l, c, r)) = Node::split_at_3(self.root(), st, ed) else { return; };
        c.lazy = Some(lazy);
        c.push();
        self.0.set(Node::merge_3(l, c, r));
    }
    
    /// 指定した位置の参照を返す。
    pub fn get(&self, i: usize) -> Option<&Ops::Value> {
        if self.len() <= i { return None; }
        let root = Node::splay_index(Node::unwrap(self.root()), i);
        self.0.set(root);
        Some(&root.value)
    }
    
    /// 指定した位置の可変参照を返す。
    pub fn get_mut(&mut self, i: usize) -> Option<RefMut<'_, Ops>> {
        if self.len() <= i { return None; }
        let root = Node::splay_index(Node::unwrap(self.root()), i);
        self.0.set(root);
        Some(RefMut(root))
    }
    
    /// `[self, ret] = [tree[..at], tree[at..]]; return ret;` をする。
    /// 
    /// # Panic
    /// 
    /// !(at <= self.len)
    pub fn split_off(&mut self, at: usize) -> Self {
        assert!(at <= self.len());
        let [left, right] = Node::split_at(self.root(), at);
        self.0.set(left);
        Self(Cell::new(right))
    }
    
    /// 木を右からマージする。
    pub fn merge(&mut self, right: Self) {
        let tmp = Node::merge(self.root(), right.root());
        self.0.set(tmp);
    }
    
    pub fn iter(&self) -> Iter<'_, Ops> {
        Iter { splay: self, st: 0, ed: self.len() }
    }
    
    pub fn range(&self, range: impl RangeBounds<usize>) -> Iter<'_, Ops> {
        let [st, ed] = range.as_bounds(self.len());
        Iter { splay: self, st, ed }
    }
}

impl<Ops: SplayOps> FromIterator<Ops::Value> for SplayTree<Ops> {
    fn from_iter<T: IntoIterator<Item = Ops::Value>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut root = if let Some(v) = iter.next() { Node::new(v) } else { return Self::new(); };
        for v in iter {
            let node = Node::new(v);
            root.parent = node;
            node.child[0] = root;
            node.update();
            root = node;
        }
        Self(Cell::new(root))
    }
}

impl<'a, Ops: SplayOps> IntoIterator for &'a SplayTree<Ops> {
    type Item = &'a Ops::Value;
    type IntoIter = Iter<'a, Ops>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<Ops: SplayOps> Clone for SplayTree<Ops> {
    fn clone(&self) -> Self { self.iter().map(|v| v.clone()).collect() }
}

impl<Ops: SplayOps> Index<usize> for SplayTree<Ops> {
    type Output = Ops::Value;
    fn index(&self, index: usize) -> &Self::Output { assert!(index < self.len()); self.get(index).unwrap() }
}

impl<Ops: SplayOps> Debug for SplayTree<Ops> where Ops::Value: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<Ops: SplayOps> Drop for SplayTree<Ops> {
    fn drop(&mut self) {}
}



/// [`SplayTree::entry`] の返り値型。
/// 
/// # Constraints
/// 
/// `entry` 対象である `self.0` が根であること。
pub struct RefMut<'a, Ops: SplayOps>(&'a mut Node<Ops>);

impl<Ops: SplayOps> Deref for RefMut<'_, Ops> {
    type Target = Ops::Value;
    fn deref(&self) -> &Self::Target { &self.0.value }
}

impl<Ops: SplayOps> DerefMut for RefMut<'_, Ops> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0.value }
}

impl<Ops: SplayOps> Drop for RefMut<'_, Ops> {
    fn drop(&mut self) { self.0.update(); }
}



/// [`SplayTree::iter`], [`SplayTree::range`] の返り値型。保持する区間は `st..ed` で表される。
/// 
/// [`Iter`] 存在中に根が変わったりするのは OK だけど、新たに値を追加するとかは NG なことに注意。
pub struct Iter<'a, Ops: SplayOps> {
    splay: &'a SplayTree<Ops>,
    st: usize,
    ed: usize
}

impl<'a, Ops: SplayOps> Iterator for Iter<'a, Ops> {
    type Item = &'a Ops::Value;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.st == self.ed { return None; }
        self.st += 1;
        Some(&self.splay[self.st-1])
    }
}

impl<'a, Ops: SplayOps> DoubleEndedIterator for Iter<'a, Ops> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.st == self.ed { return None; }
        self.ed -= 1;
        Some(&self.splay[self.ed])
    }
}





struct Node<Ops: SplayOps> {
    pub child: [*mut Self; 2],
    pub parent: *mut Self,
    pub len: usize,
    pub rev: bool,
    pub value: Ops::Value,
    pub acc: Ops::Acc,
    pub lazy: Option<Ops::Lazy>
}

impl<Ops: SplayOps> Node<Ops> {
    /// leak した `Node` を返す。
    fn new<'a>(value: Ops::Value) -> &'a mut Self {
        let tmp = Self { child: [null_mut(); 2], parent: null_mut(), len: 1, rev: false, acc: Ops::to_acc(&value), value, lazy: None };
        Box::leak(Box::new(tmp))
    }
    
    fn get<'a>(ptr: *mut Self) -> Option<&'a mut Self> { unsafe { ptr.as_mut() } }
    fn unwrap<'a>(ptr: *mut Self) -> &'a mut Self {
        debug_assert!(!ptr.is_null());
        unsafe { &mut *ptr }
    }
    
    /// 親 `p` に対する `self` の位置を返す。
    fn pos(&self, p: &Self) -> usize {
        debug_assert!(ptr_eq(p.child[0], self) || ptr_eq(p.child[1], self));
        ptr_eq(p.child[1], self) as usize
    }
    
    /// `self` が根になるよう、適切に回転させる。
    fn splay(&mut self) {
        while let Some(p) = Node::get(self.parent) {
            let pos = self.pos(p);
            
            if let Some(g) = Node::get(p.parent) {
                if pos == p.pos(g) {
                    p.rotate(g, pos); self.rotate(p, pos);
                } else {
                    self.rotate(p, pos); self.rotate(g, pos^1);
                }
            } else {
                self.rotate(p, pos);
            }
        }
    }
    
    /// `self` が自分自身の親の位置に来るように回転させる。
    fn rotate(&mut self, p: &mut Self, pos: usize) {
        self.push();
        
        // connect g --[pos]--> self
        self.parent = p.parent;
        if let Some(g) = Node::get(self.parent) { g.child[p.pos(g)] = self; }
        
        // connect p --[pos]--> self.child[pos^1]
        p.child[pos] = self.child[pos^1];
        if let Some(c) = Node::get(p.child[pos]) { c.parent = p; }
        
        // connect self --[pos^1]--> p
        p.parent = self;
        self.child[pos^1] = p;
        
        p.update(); self.update();
    }
    
    /// 遅延作用と反転を行い、子に伝搬させる。
    fn push(&mut self) {
        if let Some(lazy) = self.lazy.take() {
            Ops::act_lazy(&mut self.value, &mut self.acc, &lazy, self.len);
            for c in self.child { Self::apply_lazy(c, &lazy); }
        }
        
        if replace(&mut self.rev, false) {
            self.child.swap(0, 1);
            for c  in self.child { if let Some(c) = Node::get(c) { c.rev ^= true; } }
        }
    }
    
    /// `dir` に遅延作用 `ad` を与える。
    fn apply_lazy(dir: *mut Self, ad: &Ops::Lazy) {
        let Some(Node { lazy, .. }) = Node::get(dir) else { return; };
        if let Some(lazy) = lazy {
            Ops::comp_lazy(lazy, ad);
        } else {
            *lazy = Some(ad.clone());
        }
    }
    
    /// `self.len, self.acc` を子の情報から更新する。
    fn update(&mut self) {
        self.len = 1;
        self.acc = Ops::to_acc(&self.value);
        for pos in [0, 1] {
            if let Some(c) = Node::get(self.child[pos]) {
                c.push();
                self.len += c.len;
                self.acc = if pos == 0 { Ops::merge_acc(&c.acc, &self.acc) } else { Ops::merge_acc(&self.acc, &c.acc) };
            }
        }
    }
    
    /// 頂点 `i` を見つけて `splay` し、根である頂点 `i` を返す。
    /// 
    /// # Constraints
    /// 
    /// `i < root.len`
    fn splay_index(mut root: &mut Self, mut i: usize) -> &mut Self {
        assert!(i < root.len);
        loop {
            root.push();
            for c in root.child { if let Some(c) = Node::get(c) { c.push(); } }
            let ls = Node::get(root.child[0]).map_or(0, |left| left.len);
            
            use std::cmp::Ordering::*;
            root = match i.cmp(&ls) {
                Less => { Node::unwrap(root.child[0]) }
                Equal => { root.splay(); return root; }
                Greater => { i -= ls + 1; Node::unwrap(root.child[1]) }
            }
        }
    }
    
    /// マージした木を返す。
    /// 
    /// `l` の一番右の頂点が根になるよう `splay` した後に `r` と繋げる。
    fn merge(l: *mut Self, r: *mut Self) -> *mut Self {
        let Some(mut left) = Node::get(l) else { return r; };
        let Some(right) = Node::get(r) else { return left; };
        left = Self::splay_index(left, left.len-1);
        left.push();
        left.child[1] = right; right.parent = left;
        left.update();
        left
    }
    
    /// `tree[..at], tree[at..]` で構成される 2 つの木を返す。
    /// 
    /// # Constraints
    /// 
    /// `at <= root.len`
    fn split_at(root: *mut Self, at: usize) -> [*mut Self; 2] {
        let Some(mut root) = Node::get(root) else { return [null_mut(), null_mut()]; };
        if at == 0 { return [null_mut(), root]; }
        if at == root.len { return [root, null_mut()]; }
        root = Self::splay_index(root, at);
        root.push();
        let left = replace(&mut root.child[0], null_mut());
        if let Some(left) = Node::get(left) {
            left.parent = null_mut();
            root.update();
        }
        [left, root]
    }
    
    /// `tree[..at1], tree[at1..at2], tree[at2..]` で構成される 2 つの木を返す。
    /// 
    /// # Constraints
    /// 
    /// `at1 <= at2 <= root.len`
    fn split_at_3<'a>(root: *mut Self, at1: usize, at2: usize) -> Option<(*mut Self, &'a mut Self, *mut Self)> {
        if at1 == at2 { return None; }
        let [c, r] = Self::split_at(root, at2);
        let [l, c] = Self::split_at(c, at1);
        Some((l, Node::unwrap(c), r))
    }
    
    /// マージした木を返す。
    fn merge_3(l: *mut Self, c: *mut Self, r: *mut Self) -> *mut Self {
        Self::merge(Self::merge(l, c), r)
    }
}
